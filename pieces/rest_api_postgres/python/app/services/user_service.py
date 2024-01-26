from __future__ import annotations

from app.db import db
from app.exceptions import NoUserDeletedError, UserCreationError
from app.models.user import AdminUserUpdate, User, UserCreate, UserInDb, UserUpdate
from app.security import get_password_hash


async def create_user(user: UserCreate) -> User:
    hashed_password = get_password_hash(user.password)
    async with db.pool.acquire() as conn:
        await conn.execute(
            """
            INSERT INTO users(user_name, hashed_password, is_admin)
            VALUES ($1, $2, False);
            """,
            user.user_name,
            hashed_password,
        )

        created_user = await get_user_by_user_name(user.user_name)

        if not created_user:
            raise UserCreationError("User not created")

        return created_user


async def delete_user(user_id: int) -> None:
    async with db.pool.acquire() as conn:
        result = await conn.execute(
            """
            DELETE FROM users
            WHERE id = $1;
            """,
            user_id,
        )

    if result != "DELETE 1":
        raise NoUserDeletedError()


async def get_all_users() -> list[User] | None:
    async with db.pool.acquire() as conn:
        users = await conn.fetch(
            """
            SELECT id, user_name, is_admin
            FROM users;
            """
        )

        if not users:
            return None

        return [User(**user) for user in users]


async def get_user(user_id: int) -> User | None:
    async with db.pool.acquire() as conn:
        user = await conn.fetchrow(
            """
            SELECT id, user_name, is_admin
            FROM users WHERE id = $1;
            """,
            user_id,
        )

    if not user:
        return None

    return User(**user)


async def get_user_by_user_name(user_name: str) -> User | None:
    async with db.pool.acquire() as conn:
        user = await conn.fetchrow(
            """
            SELECT id, user_name, is_admin
            FROM users
            WHERE user_name = $1;
            """,
            user_name,
        )

    if not user:
        return None

    return User(**user)


async def get_full_user_by_username(user_name: str) -> UserInDb | None:
    async with db.pool.acquire() as conn:
        user = await conn.fetchrow(
            """
            SELECT id, user_name, hashed_password, is_admin
            FROM users WHERE user_name = $1;
            """,
            user_name,
        )

    if not user:
        return None

    return UserInDb(**user)


async def update_user(user: AdminUserUpdate | UserUpdate) -> User:
    if isinstance(user, AdminUserUpdate):
        is_admin = user.is_admin
    else:
        is_admin = False

    async with db.pool.acquire() as conn:
        if not user.password:
            await conn.execute(
                """
                UPDATE users
                SET user_name = $1, is_admin = $2
                WHERE id = $3;
                """,
                user.user_name,
                is_admin,
                user.id,
            )
        else:
            hashed_password = get_password_hash(user.password)
            await conn.execute(
                """
                UPDATE users
                SET user_name = $1, hashed_password = $2, is_admin = $3
                WHERE id = $4;
                """,
                user.user_name,
                hashed_password,
                is_admin,
                user.id,
            )

        return User(**user.model_dump())
