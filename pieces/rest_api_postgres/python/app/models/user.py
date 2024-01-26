from typing import Union

from camel_converter.pydantic_base import CamelBase


class _BaseUser(CamelBase):
    user_name: str


class User(_BaseUser):
    id: int
    is_admin: bool = False


class UserCreate(_BaseUser):
    password: str


class UserUpdate(_BaseUser):
    id: int
    password: Union[str, None] = None


class AdminUserUpdate(UserUpdate):
    is_admin: bool = False


class UserInDb(_BaseUser):
    id: int
    hashed_password: str
    is_admin: bool = False
