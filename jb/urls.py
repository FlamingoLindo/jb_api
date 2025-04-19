from django.urls import path
from rest_framework_simplejwt.views import TokenRefreshView

from .views.auth import MyTokenObtainPairView, create_user, login
from .views.users import get_users, user_detail
from .views.brand import create_brand, get_brand, manage_brand, get_brand_by_id
from .views.product import create_product, get_product, manage_product, reajust_price, restore_price, get_product_by_brand
from .views.item import create_item, get_item, manage_item, filter_items, get_item_classes, get_item_types


urlpatterns = [
    # Auth URLs
    path('token/', MyTokenObtainPairView.as_view(), name='token_obtain_pair'),
    path('token/refresh/', TokenRefreshView.as_view(), name='token_refresh'),

    # User URLs
    path('users/', get_users, name='get_users'),
    path('users/create/', create_user, name='create_user'),
    path('user/<int:pk>/', user_detail, name='user_detail'),
    path('login/', login, name='login'),


    # Brand URLs
    path('create_brand/', create_brand, name='create_brand'),
    path('get_brand/', get_brand, name='get_brand'),
    path('manage_brand/<int:pk>/', manage_brand, name='manage_brand'),
    path('get_brand_by_id/<int:pk>/', get_brand_by_id, name='get_brand_by_id'),

    # Product URLs
    path('create_product/', create_product, name='create_product'),
    path('get_product/', get_product, name='get_product'),
    path('get_product_by_brand/<int:brand_id>/', get_product_by_brand, name='get_product_by_brand'),
    path('manage_product/<int:pk>/', manage_product, name='manage_product'),
    path('reajust_price/<int:brand_id>/<int:reajust_value>/', reajust_price, name='reajust_price'),
    path('restore_price/<int:brand_id>/', restore_price, name='restore_price'),

    # Item URLs
    path('create_item/', create_item, name='create_item'),
    path('get_item/', get_item, name='get_item'),
    path('manage_item/<int:pk>/', manage_item, name='manage_item'),
    path('filter_items/', filter_items, name='filter_items'),
    path('get_item_classes/', get_item_classes, name='get_item_classes'),
    path('get_item_types/', get_item_types, name='get_item_types'),
] 