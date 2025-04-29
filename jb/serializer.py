from rest_framework import serializers
from rest_framework.validators import UniqueValidator
from .models import Product, Brand, Item, CustomUser

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

class CustomUserSerializer(serializers.ModelSerializer):
    password = serializers.CharField(
        write_only=True,
        required=True,
        min_length=8,
        style={'input_type': 'password'}
    )

    class Meta:
        model = CustomUser
        fields = ['id', 'email', 'name', 'password', 'is_staff', 'is_active']

    def create(self, validated_data):
        password = validated_data.pop('password', None)
        instance = self.Meta.model(**validated_data)
        if password:
            instance.set_password(password)
        instance.save()
        return instance

    def update(self, instance, validated_data):
        password = validated_data.pop('password', None)
        if password:
            instance.set_password(password)
        return super().update(instance, validated_data)