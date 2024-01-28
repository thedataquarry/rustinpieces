import pytest


async def test_add_book(test_client, book_json):
    response = await test_client.post("book/", json=book_json)
    response_json = response.json()
    assert "id" in response_json
    del response_json["id"]

    assert response_json == book_json


@pytest.mark.usefixtures("book_in_db")
async def test_add_book_duplicate(test_client, book_json):
    response = await test_client.post("book/", json=book_json)

    assert response.status_code == 400
    assert "already exists" in response.json()["detail"]


async def test_get_book_by_id(test_client, book_in_db):
    response = await test_client.get(f"book/{book_in_db.id}")
    expected = book_in_db.model_dump(by_alias=True)
    expected["bookStatus"] = expected["bookStatus"].value
    expected["dateAdded"] = expected["dateAdded"].isoformat().replace("+00:00", "Z")
    expected["dateRead"] = expected["dateRead"].isoformat().replace("+00:00", "Z")

    assert response.json() == expected


async def test_get_book_by_id_none(test_client):
    response = await test_client.get("book/1")

    assert response.status_code == 404


async def test_get_books(test_client, book_in_db):
    response = await test_client.get("book")
    expected = book_in_db.model_dump(by_alias=True)
    expected["bookStatus"] = expected["bookStatus"].value
    expected["dateAdded"] = expected["dateAdded"].isoformat().replace("+00:00", "Z")
    expected["dateRead"] = expected["dateRead"].isoformat().replace("+00:00", "Z")

    assert response.json() == [expected]


async def test_get_books_none(test_client):
    response = await test_client.get("book")

    assert response.status_code == 404


async def test_update_book(test_client, book_in_db):
    book_json = book_in_db.model_dump(by_alias=True)
    book_json["title"] = "new title"
    book_json["bookStatus"] = book_json["bookStatus"].value
    book_json["dateAdded"] = book_json["dateAdded"].isoformat().replace("+00:00", "Z")
    book_json["dateRead"] = book_json["dateRead"].isoformat().replace("+00:00", "Z")
    response = await test_client.put("book", json=book_json)

    assert response.json()["title"] == "new title"


async def test_update_book_duplicate(test_client, book_in_db):
    book_json = book_in_db.model_dump(by_alias=True)
    book_json["title"] = "new title"
    book_json["bookStatus"] = book_json["bookStatus"].value
    book_json["dateAdded"] = book_json["dateAdded"].isoformat().replace("+00:00", "Z")
    book_json["dateRead"] = book_json["dateRead"].isoformat().replace("+00:00", "Z")
    response = await test_client.post("book/", json=book_json)

    assert response.json()["title"] == "new title"
    response = await test_client.put("book", json=book_json)

    assert response.status_code == 400
    assert "already exists" in response.json()["detail"]
