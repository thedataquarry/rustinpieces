from uuid import uuid4

import pytest
import srsly
from main import app
from typer.testing import CliRunner


@pytest.fixture
def index_name():
    return str(uuid4())


@pytest.fixture
def wine_data():
    return [
        {
            "title": "Nicosia 2013 Vulkà Bianco  (Etna)",
            "description": "Aromas include tropical fruit, broom, brimstone and dried herb. The palate isn't overly expressive, offering unripened apple, citrus and dried sage alongside brisk acidity.",
            "taster_name": "Kerin O’Keefe",
            "designation": "Vulkà Bianco",
            "variety": "White Blend",
            "country": "Italy",
            "winery": "Nicosia",
            "id": 1,
            "points": "87",
            "taster_twitter_handle": "@kerinokeefe",
            "price": None,
            "region_1": "Etna",
            "region_2": None,
            "province": "Sicily & Sardinia",
        },
        {
            "title": "Quinta dos Avidagos 2011 Avidagos Red (Douro)",
            "description": "This is ripe and fruity, a wine that is smooth while still structured. Firm tannins are filled out with juicy red berry fruits and freshened with acidity. It's  already drinkable, although it will certainly be better from 2016.",
            "taster_name": "Roger Voss",
            "designation": "Avidagos",
            "variety": "Portuguese Red",
            "country": "Portugal",
            "winery": "Quinta dos Avidagos",
            "id": 2,
            "points": "87",
            "taster_twitter_handle": "@vossroger",
            "price": 15,
            "region_1": None,
            "region_2": None,
            "province": "Douro",
        },
        {
            "title": "Rainstorm 2013 Pinot Gris (Willamette Valley)",
            "description": "Tart and snappy, the flavors of lime flesh and rind dominate. Some green pineapple pokes through, with crisp acidity underscoring the flavors. The wine was all stainless-steel fermented.",
            "taster_name": "Paul Gregutt",
            "designation": None,
            "variety": "Pinot Gris",
            "country": "US",
            "winery": "Rainstorm",
            "id": 3,
            "points": "87",
            "taster_twitter_handle": "@paulgwine\xa0",
            "price": 14,
            "region_1": "Willamette Valley",
            "region_2": "Willamette Valley",
            "province": "Oregon",
        },
        {
            "title": "St. Julian 2013 Reserve Late Harvest Riesling (Lake Michigan Shore)",
            "description": "Pineapple rind, lemon pith and orange blossom start off the aromas. The palate is a bit more opulent, with notes of honey-drizzled guava and mango giving way to a slightly astringent, semidry finish.",
            "taster_name": "Alexander Peartree",
            "designation": "Reserve Late Harvest",
            "variety": "Riesling",
            "country": "US",
            "winery": "St. Julian",
            "id": 4,
            "points": "87",
            "taster_twitter_handle": None,
            "price": 13,
            "region_1": "Lake Michigan Shore",
            "region_2": None,
            "province": "Michigan",
        },
        {
            "title": "Sweet Cheeks 2012 Vintner's Reserve Wild Child Block Pinot Noir (Willamette Valley)",
            "description": "Much like the regular bottling from 2012, this comes across as rather rough and tannic, with rustic, earthy, herbal characteristics. Nonetheless, if you think of it as a pleasantly unfussy country wine, it's a good companion to a hearty winter stew.",
            "taster_name": "Paul Gregutt",
            "designation": "Vintner's Reserve Wild Child Block",
            "variety": "Pinot Noir",
            "country": "US",
            "winery": "Sweet Cheeks",
            "id": 5,
            "points": "87",
            "taster_twitter_handle": "@paulgwine\xa0",
            "price": 65,
            "region_1": "Willamette Valley",
            "region_2": "Willamette Valley",
            "province": "Oregon",
        },
    ]


def test_create_index(client, index_name):
    CliRunner().invoke(app, ["create-index", "-i", index_name])
    index = client.get_index(index_name)
    assert index is not None
    searchable_attributes = index.get_searchable_attributes()
    assert searchable_attributes == [
        "title",
        "description",
        "taster_name",
        "designation",
        "variety",
        "province",
        "country",
        "winery",
    ]


def test_index_data(client, tmp_path, wine_data, index_name):
    index = client.create_index(index_name)
    file_path = tmp_path / "data.jsonl.gz"
    srsly.write_gzip_jsonl(file_path, wine_data)
    CliRunner().invoke(app, ["index-data", "-d", str(file_path), "-w", "-i", index_name])
    documents = index.get_documents()
    assert documents.total == 5


def test_search(wine_data, client, index_name):
    index = client.create_index(index_name)
    task = index.add_documents(wine_data)
    client.wait_for_task(task.task_uid)
    result = CliRunner().invoke(app, ["search", "Nicosia", "-i", index_name])
    out = result.stdout

    assert "'title': 'Nicosia 2013 Vulkà Bianco" in out
