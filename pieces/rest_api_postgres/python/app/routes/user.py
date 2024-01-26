from typing import List

from asyncpg.exceptions import UniqueViolationError
from fastapi import HTTPException
from starlette.status import (
    HTTP_204_NO_CONTENT,
    HTTP_400_BAD_REQUEST,
    HTTP_403_FORBIDDEN,
    HTTP_404_NOT_FOUND,
    HTTP_500_INTERNAL_SERVER_ERROR,
)

from app.config import config
from app.deps import CurrentAdminUser, CurrentUser, logger
from app.exceptions import NoUserDeletedError
from app.models.user import AdminUserUpdate, User, UserCreate, UserUpdate
from app.services.user_service import create_user as create_user_service
from app.services.user_service import delete_user as delete_user_service
from app.services.user_service import get_all_users as get_all_users_service
from app.services.user_service import get_user as get_user_service
from app.services.user_service import update_user as update_user_service
from app.utils import APIRouter

router = APIRouter(tags=["User"], prefix=f"{config.API_PREFIX}/user")


@router.post("/")
async def create_user(user: UserCreate) -> User:
    """Create a new user."""
    logger.info("Creating user")
    try:
        created_user = await create_user_service(user)
    except UniqueViolationError:
        logger.info("A user with the user name %s already exists", user.user_name)
        raise HTTPException(
            status_code=HTTP_400_BAD_REQUEST,
            detail=f"A user with the user name {user.user_name} already exists",
        )
    except Exception as e:
        logger.error("An error occurred while inserting user: %s", e)
        raise HTTPException(
            status_code=HTTP_500_INTERNAL_SERVER_ERROR,
            detail="An error occurred while creating the user",
        )

    return created_user


@router.delete("/me", status_code=HTTP_204_NO_CONTENT)
async def delete_me(user: CurrentUser) -> None:
    """Delete the currently logged in user."""
    try:
        logger.info("Deleting user with ID %i", user.id)
        await delete_user_service(user.id)
    except NoUserDeletedError:
        logger.info("No user deleted")
        raise HTTPException(HTTP_400_BAD_REQUEST, detail="No user deleted")


@router.delete("/admin/{user_id}", status_code=HTTP_204_NO_CONTENT)
async def admin_delete_user(user_id: int, _: CurrentAdminUser) -> None:
    """Delete a user as an admin."""
    logger.info("Checking if user with ID %i exists", user_id)
    user = await get_user_service(user_id)

    if not user:
        logger.info("No user with the ID %i found", user_id)
        raise HTTPException(HTTP_400_BAD_REQUEST, detail=f"No user with the ID {user_id} found")

    try:
        logger.info("Deleting user with ID %i", user_id)
        await delete_user_service(user_id)
    except NoUserDeletedError:
        logger.info("No user deleted")
        raise HTTPException(HTTP_400_BAD_REQUEST, detail="No user deleted")


@router.get("/")
async def get_users(_: CurrentAdminUser) -> List[User]:
    """Get all users."""
    logger.info("Getting all users")
    users = await get_all_users_service()

    if not users:
        logger.info("No users found")
        raise HTTPException(HTTP_404_NOT_FOUND, detail="No users found")

    return users


@router.get("/{user_id}")
async def get_user(user_id: int, _: CurrentAdminUser) -> User:
    """Get a user by ID."""
    logger.info("Getting user by ID: %i", user_id)
    user = await get_user_service(user_id)

    if not user:
        logger.info("No user with ID %i found", user_id)
        raise HTTPException(HTTP_404_NOT_FOUND, detail=f"No user with ID {user_id} found")

    return user


@router.get("/me")
async def get_me(current_user: CurrentUser) -> User:
    """Get the currently logged in user's information."""
    return current_user


@router.put("/me")
async def update_me(current_user: CurrentUser, user: UserUpdate) -> User:
    """Update the currently logged in user."""
    if user.id != current_user.id:
        logger.info("Cannot update another user")
        raise HTTPException(HTTP_403_FORBIDDEN, detail="Cannot update another user")

    return await update_user_service(user)


@router.put("/admin")
async def admin_update_user(_: CurrentAdminUser, user: AdminUserUpdate) -> User:
    """Update a user as an administrator."""
    return await update_user_service(user)
