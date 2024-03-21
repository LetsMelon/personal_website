build:
	docker build -t melcher_io_website:latest .

run:
	docker run --name melcher_io_website -d --rm -p 8080:80 -p 80:80 -p 443:80 melcher_io_website:latest

stop:
	docker stop melcher_io_website

restart: build stop run
