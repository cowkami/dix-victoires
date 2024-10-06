BUILD := interim

run:
	trunk serve

$(BUILD):
	mkdir -p $(BUILD)

address.csv: $(BUILD)
	@echo "Downloading address database..."
	wget --quiet https://www.post.japanpost.jp/zipcode/dl/roman/KEN_ALL_ROME.zip -O $(BUILD)/KEN_ALL_ROME.zip
	@echo "Download completed!\n"

	@echo "Unzipping address database..."
	unzip -o $(BUILD)/KEN_ALL_ROME.zip -d $(BUILD)
	mv $(BUILD)/KEN_ALL_ROME.csv $(BUILD)/address_sjis.csv
	@echo "Unzipping completed!\n"

	@echo "\nConverting address database encoding from Shift-JIS to UTF-8..."
	iconv -f SHIFT-JIS -t UTF-8 $(BUILD)/address_sjis.csv > $(BUILD)/address.csv
	@echo "Conversion completed!\n"

.PHONY: build
build: address.csv
	@echo "Building app..."
	trunk build --release
	@echo "Build completed!\n"

deploy: build
	@echo "Deploying app..."
	gsutil rsync -dr dist gs://dix-victoires
	@echo "Deploy completed!\n"

.PHONY: clean
clean:
	@echo "Cleaning..."
	rm -rf $(BUILD)
	rm -rf dist
	rm -rf target
	@echo "Clean completed!\n"
