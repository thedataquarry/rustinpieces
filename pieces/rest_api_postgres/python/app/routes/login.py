from datetime import timedelta
from typing import Annotated

from fastapi import Depends, HTTPException
from fastapi.security import OAuth2PasswordRequestForm
from starlette.status import HTTP_400_BAD_REQUEST

from app.config import config
from app.deps import Config, CurrentUser, logger
from app.models.token import Token
from app.models.user import User
from app.security import create_access_token, verify_password
from app.services.user_service import get_full_user_by_username
from app.utils import APIRouter

router = APIRouter(tags=["Login"], prefix=f"{config.API_PREFIX}/login")


@router.post("/access-token")
async def login_access_token(
    form_data: Annotated[OAuth2PasswordRequestForm, Depends()], config: Config
) -> Token:
    """OAuth2 compatible token login, get an access token for future requests."""
    logger.info("Logging user in.")
    user = await get_full_user_by_username(form_data.username)

    if not user:
        logger.info("User not found")
        raise HTTPException(
            status_code=HTTP_400_BAD_REQUEST, detail="Incorrect user name or password"
        )

    if not verify_password(form_data.password, user.hashed_password):
        logger.info("Incorrect password")
        raise HTTPException(
            status_code=HTTP_400_BAD_REQUEST, detail="Incorrect user name or password"
        )

    access_token_expires = timedelta(minutes=config.ACCESS_TOKEN_EXPIRE_MINUTES)

    return Token(
        access_token=create_access_token(user.id, expires_delta=access_token_expires),
        token_type="Bearer",
    )


@router.post("/test-token")
def test_token(current_user: CurrentUser) -> User:
    """Test access token."""
    return current_user
