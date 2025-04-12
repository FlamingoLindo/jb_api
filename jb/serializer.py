from rest_framework import serializers
from rest_framework.validators import UniqueValidator
from .models import Product, Brand, Item

class BrandSerializer(serializers.ModelSerializer):
    name = serializers.CharField(
        max_length=100,
        validators=[
            UniqueValidator(
                queryset=Brand.objects.all(),
                message="Marca com mesmo nome já existe!"
            )
        ]
    )
    class Meta:
        model = Brand
        fields = '__all__'

class ProductSerializer(serializers.ModelSerializer):
    code = serializers.CharField(
        max_length=200,
        validators=[
            UniqueValidator(
                queryset=Product.objects.all(),
                message="Produco com mesmo código já existe!"
            )
        ]
    )

    class Meta:
        model = Product
        fields = '__all__'

class ItemSerializer(serializers.ModelSerializer):

    class Meta:
        model = Item
        fields = '__all__'
