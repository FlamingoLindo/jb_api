from decimal import Decimal, InvalidOperation

from django.shortcuts import get_object_or_404
from django.db.models import F, ExpressionWrapper, DecimalField
from rest_framework.decorators import api_view
from rest_framework.response import Response
from rest_framework import status
from ..models import Product, Brand
from ..serializer import ProductSerializer, BrandSerializer

@api_view(['POST'])
def create_product(request):
    serializer = ProductSerializer(data=request.data)
    if serializer.is_valid():
        serializer.save()
        return Response(serializer.data, status=status.HTTP_201_CREATED)
    return Response(serializer.errors, status=status.HTTP_400_BAD_REQUEST)

@api_view(['GET'])
def get_product(request):
    products = Product.objects.all()
    serializer = ProductSerializer(products, many=True)
    return Response(serializer.data, status=status.HTTP_200_OK)

@api_view(["GET"])
def get_product_by_brand(request, brand_id):
    brand = get_object_or_404(Brand, pk=brand_id)

    products = Product.objects.filter(brand=brand)

    return Response(
        {
            "brand": BrandSerializer(brand).data,
            "products": ProductSerializer(products, many=True).data,
        },
        status=status.HTTP_200_OK,
    )

@api_view(['GET', 'PUT', 'DELETE'])
def manage_product(request, pk):
    product = get_object_or_404(Product, pk=pk)

    if request.method == 'GET':
        return Response(ProductSerializer(product).data, status=status.HTTP_200_OK)

    elif request.method == 'PUT':
        serializer = ProductSerializer(product, data=request.data)
        if serializer.is_valid():
            serializer.save()
            return Response(serializer.data, status=status.HTTP_200_OK)
        return Response(serializer.errors, status=status.HTTP_400_BAD_REQUEST)

    elif request.method == 'DELETE':
        product.delete()
        return Response(status=status.HTTP_204_NO_CONTENT)
    
@api_view(['PUT'])
def reajust_price(request, brand_id, reajust_value):
    brand = get_object_or_404(Brand, pk=brand_id)
    products = Product.objects.filter(brand=brand)
    
    try:
        multiplier = Decimal(reajust_value) / Decimal(100) + Decimal(1)
    except (InvalidOperation, ValueError):
        return Response({"error": "Valor de reajuste inválido"}, status=status.HTTP_400_BAD_REQUEST)
    
    # Update all prices at once using an expression.
    products.update(
        price=ExpressionWrapper(F('original_price') * multiplier, output_field=DecimalField())
    )
    
    # Refresh the products from the database so we have updated values.
    products = Product.objects.filter(brand=brand)
    serializer = ProductSerializer(products, many=True)
    return Response(serializer.data, status=status.HTTP_200_OK)

@api_view(['PUT'])
def restore_price(request, brand_id):
    brand= get_object_or_404(Brand, pk=brand_id)
    products = Product.objects.filter(brand=brand)

    # Set price to original_price for all products of the brand
    products.update(price=F('original_price'))

    products = Product.objects.filter(brand=brand)
    serializer = ProductSerializer(products, many=True)
    return Response(serializer.data, status=status.HTTP_200_OK)

