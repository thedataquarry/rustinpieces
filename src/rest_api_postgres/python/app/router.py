from app.routes import book
from app.utils import APIRouter

api_router = APIRouter()
api_router.include_router(book.router)
