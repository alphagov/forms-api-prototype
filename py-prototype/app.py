#!/usr/bin/env python3

from typing import Dict

from flask import Flask
from flask_restful import Resource, Api  # type: ignore

app = Flask(__name__)
api = Api(app)


class HelloWorld(Resource):
    def get(self) -> Dict[str, str]:
        return {"hello": "forms"}


api.add_resource(HelloWorld, "/")

if __name__ == "__main__":
    app.run(debug=True)
