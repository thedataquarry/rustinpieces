from app.routes import login, user
from app.utils import APIRouter

api_router = APIRouter()
api_router.include_router(login.router)
api_router.include_router(user.router)
