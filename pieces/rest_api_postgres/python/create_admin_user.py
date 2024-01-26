import asyncio

from asyncpg.exceptions import UniqueViolationError

from app.db import db
from app.models.user import AdminUserUpdate, UserCreate
from app.services.user_service import create_user, get_user_by_user_name, update_user


async def main() -> int:
    """Create a dummy admin user for testing purposes."""
    try:
        await db.create_pool()
        user = UserCreate(user_name="admin", is_admin=True, password="admin")
        await create_user(user)
        db_user = await get_user_by_user_name("admin")
        if not db_user:
            raise ValueError("Error creating user")

        updated_user = AdminUserUpdate(id=db_user.id, user_name=db_user.user_name, is_admin=True)
        await update_user(updated_user)
        print("Admin user created")
    except UniqueViolationError:
        print("User already created")
    finally:
        await db.close_pool()

    return 0


if __name__ == "__main__":
    SystemExit(asyncio.run(main()))
