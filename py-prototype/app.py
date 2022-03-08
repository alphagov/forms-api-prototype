#!/usr/bin/env python3

from typing import Dict, List

from flask import Flask
from flask_restful import Resource, Api  # type: ignore

app = Flask(__name__)
api = Api(app)


def forms_for_user(user) -> List[Dict[str, str]]:
    """
    Query postgres for all the forms for this username
    """
    return [{"key": "b", "display_name": "c"}]


class Published(Resource):
    # def get(self) -> Dict[str, str]:
    # return {"hello": "forms"}

    def get(self):
        user = "Alice"
        forms = []

        for form in forms_for_user(user):
            forms.append(
                {
                    "Key": form["key"],
                    "DisplayName": form["display_name"],
                    "FeedbackForm": False,
                }
            )

        return forms


api.add_resource(Published, "/published")

if __name__ == "__main__":
    app.run(debug=True)
