from django.urls import path

from .views.brand import create_brand, get_brand, manage_brand
from .views.product import create_product, get_product, manage_product, reajust_price, restore_price, get_product_by_brand
from .views.item import create_item, get_item, manage_item, filter_items


urlpatterns = [
    # Brand URLs
    path('create_brand/', create_brand, name='create_brand'),
    path('get_brand/', get_brand, name='get_brand'),
    path('manage_brand/<int:pk>/', manage_brand, name='manage_brand'),

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
] 