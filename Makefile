build:
	docker build -t melcher_io_website:latest .

run:
	docker run --name melcher_io_website -d --rm -p 8080:80 melcher_io_website:latest

stop:
	docker stop melcher_io_website

restart: stop build run
