#!/usr/bin/env python3

from typing import Dict, List

from sqlalchemy import (  # type: ignore
    Table,
    Column,
    Integer,
    String,
    MetaData,
    create_engine,
)
from sqlalchemy_json import MutableJson  # type: ignore
from flask import Flask
from flask_restful import Resource, Api  # type: ignore

app = Flask(__name__)
api = Api(app)

meta = MetaData()

db = create_engine(
    "postgresql://postgres:password@localhost/postgres", echo=True
).connect()

forms = Table(
    "forms",
    meta,
    Column("id", Integer, primary_key=True),
    Column("username", String),
    Column("key", String),
    Column("display_name", String),
    Column("form", MutableJson),
)


def forms_for_user(user) -> List[Dict[str, str]]:
    """
    Query postgres for all the forms for this username
    """
    rows = []
    for row in db.execute(forms.select().where(forms.c.username == user)):
        rows.append(
            {
                "id": row[0],
                "username": row[1],
                "key": row[2],
                "display_name": row[3],
                "form": row[4],
            }
        )
    return rows


print(forms_for_user("tris"))


class Published(Resource):
    # def get(self) -> Dict[str, str]:
    # return {"hello": "forms"}

    def get(self):
        user = "tris"
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

