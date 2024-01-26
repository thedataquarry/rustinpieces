import logging
from typing import Annotated

import jwt
from asyncpg import Pool
from fastapi import Depends, HTTPException
from fastapi.security import OAuth2PasswordBearer
from jwt.exceptions import PyJWTError
from pydantic import ValidationError
from starlette.status import HTTP_403_FORBIDDEN, HTTP_404_NOT_FOUND

from app.config import Settings, config
from app.db import db
from app.models.token import TokenPayload
from app.models.user import User
from app.security import ALGORITHM
from app.services.user_service import get_user

logging.basicConfig(format="%(asctime)s - %(levelname)s - [%(filename)s:%(lineno)d] - %(message)s")
logging.root.setLevel(level=config.log_level)
logger = logging
oauth2_scheme = OAuth2PasswordBearer(tokenUrl=f"{config.API_PREFIX}/login/access-token")


def get_config() -> Settings:
    return config


def db_pool() -> Pool:
    return db.pool


async def get_current_admin_user(token: Annotated[str, Depends(oauth2_scheme)]) -> User:
    try:
        payload = jwt.decode(token, config.SECRET_KEY, algorithms=[ALGORITHM])
        token_data = TokenPayload(**payload)
    except (PyJWTError, ValidationError) as e:
        logger.info("Could not validate credentials: %s", e)
        raise HTTPException(status_code=HTTP_403_FORBIDDEN, detail="Could not validate credentials")

    if not token_data.sub:
        logger.info("No sub present in token")
        raise HTTPException(status_code=HTTP_403_FORBIDDEN, detail="Invalid token")

    try:
        user_id = int(token_data.sub)
    except ValueError:
        logger.info("%s is not a valid user id", token_data.sub)
        raise HTTPException(
            status_code=HTTP_403_FORBIDDEN, detail=f"{token_data.sub} is not a valid ID format"
        )
    user = await get_user(user_id)

    if not user:
        logger.info("User not found")
        raise HTTPException(status_code=HTTP_404_NOT_FOUND, detail="User not found")

    print("HI")
    print(user)
    if not user.is_admin:
        logger.info("User is not an admin")
        raise HTTPException(
            status_code=HTTP_403_FORBIDDEN, detail="User does not have the required permissions"
        )

    return user


async def get_current_user(token: Annotated[str, Depends(oauth2_scheme)]) -> User:
    try:
        payload = jwt.decode(token, config.SECRET_KEY, algorithms=[ALGORITHM])
        token_data = TokenPayload(**payload)
    except (PyJWTError, ValidationError) as e:
        logger.info("Could not validate credentials: %s", e)
        raise HTTPException(status_code=HTTP_403_FORBIDDEN, detail="Could not validate credentials")

    if not token_data.sub:
        logger.info("No sub present in token")
        raise HTTPException(status_code=HTTP_403_FORBIDDEN, detail="Invalid token")

    try:
        user_id = int(token_data.sub)
    except ValueError:
        logger.info("%s is not a valid user id", token_data.sub)
        raise HTTPException(
            status_code=HTTP_403_FORBIDDEN, detail=f"{token_data.sub} is not a valid ID format"
        )
    user = await get_user(user_id)

    if not user:
        logger.info("User not found")
        raise HTTPException(status_code=HTTP_404_NOT_FOUND, detail="User not found")

    user = await get_user(user_id)

    if not user:
        logger.info("User not found")
        raise HTTPException(status_code=HTTP_404_NOT_FOUND, detail="User not found")

    return user


Config = Annotated[Settings, Depends(get_config)]
CurrentUser = Annotated[User, Depends(get_current_user)]
CurrentAdminUser = Annotated[User, Depends(get_current_admin_user)]
DbPool = Annotated[Pool, Depends(db_pool)]
