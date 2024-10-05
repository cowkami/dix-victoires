BUILD := interim
DB_CSV := $(BUILD)/address.csv
DB_JSON := $(BUILD)/address.json

run:
	trunk serve

$(BUILD):
	mkdir -p $(BUILD)

$(DB_CSV): $(BUILD)
	@echo "Downloading address database..."
	wget --quiet https://www.post.japanpost.jp/zipcode/dl/kogaki/zip/ken_all.zip -O $(BUILD)/KEN_ALL.zip
	unzip -o $(BUILD)/KEN_ALL.zip -d $(BUILD)
	mv $(BUILD)/KEN_ALL.CSV $(BUILD)/address.csv

$(DB_JSON): $(DB_CSV)
	@echo "Building address database..."
	docker build -t address-processor -f docker/Dockerfile.db .
	docker run --rm -it -v ${PWD}/$(BUILD):/work address-processor ken-all address address.csv -t json > $(BUILD)/address.jsonl

build: $(DB_JSON)
	@echo "Build completed!"

clean:
	rm -rf $(BUILD)
	docker rmi address-processor

