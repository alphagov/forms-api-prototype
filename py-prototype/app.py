#!/usr/bin/env python3

from typing import Dict, List
import json

import sqlalchemy  # type: ignore
from sqlalchemy import (  # type: ignore
    Table,
    Column,
    Integer,
    String,
    MetaData,
    create_engine,
)
from sqlalchemy_json import MutableJson  # type: ignore
from flask import Flask, request
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
        print(row[0])
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


def form_by_id_for_user(form_id: str, user: str) -> Dict[str, object]:
    """
    Query postgres for form by id for this username
    """

    row = db.execute(
        forms.select().where(
            sqlalchemy.and_(forms.c.username == user, forms.c.key == form_id)
        )
    ).fetchone()
    if not row:
        return {}
    form = {
        "id": row[0],
        "username": row[1],
        "key": row[2],
        "display_name": row[3],
        "form": row[4],
    }
    return form


def form_exists_for_user(user: str, form_id: str) -> bool:
    """
    Query postgres for form by id for this username
    """

    row = db.execute(
        forms.select().where(
            sqlalchemy.and_(forms.c.username == user, forms.c.key == form_id)
        )
    ).fetchall()

    return len(row) > 0


def update_form_for_user(user: str, form: str):
    db.execute(
        forms.update()
        .where(forms.c.username == user, forms.c.key == json.loads(form)["id"])
        .values(form=form)
    )


# TODO
def insert_form_for_user(user: str, form: str):
    db.execute(
        forms.update()
        .where(forms.c.username == user, forms.c.key == json.loads(form)["id"])
        .values(form=form)
    )


class Published(Resource):
    def get(self) -> List[Dict[str, object]]:
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


class Publish(Resource):
    """
    curl http://localhost:5000/publish \
    -d "data={'id': 'report-a-terrorist', 'configuration': '{}'}" -X POST
    """

    def post(self):
        user = "tris"
        request_body = json.loads(request.form["data"])

        form_id = request_body["id"]
        config = request_body["configuration"] if request_body["configuration"] else {}
        if form_exists_for_user(user, form_id):
            update_form_for_user(user, config)
        else:
            insert_form_for_user(user, config)
        return config


class PublishedId(Resource):
    def get(self, form_id) -> Dict[str, object]:
        user = "tris"

        return form_by_id_for_user(form_id, user)


api.add_resource(Published, "/published")
api.add_resource(PublishedId, "/published/<string:form_id>")
api.add_resource(Publish, "/publish")

if __name__ == "__main__":
    app.run(debug=True)
