##
# Python prototype
#

default: lints run

run:
	@poetry run python app.py

watch:
	@poetry run watchmedo auto-restart --ignore-directories --patterns="*.py" --ignore-patterns="*#*" --recursive make

lints:
	clear
	@poetry run black app.py
	@poetry run flake8 app.py
	@poetry run mypy app.py

setup:
	@which poetry || curl -sSL https://install.python-poetry.org | python3 -
	@poetry self update --preview
	@poetry update

# end
