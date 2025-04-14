from django.shortcuts import get_object_or_404
from django.db.models import Avg
from rest_framework.decorators import api_view
from rest_framework.response import Response
from rest_framework import status
from ..models import Item
from ..serializer import ItemSerializer

@api_view(['POST'])
def create_item(request):
    serializer = ItemSerializer(data=request.data)
    if serializer.is_valid():
        serializer.save()
        return Response(serializer.data, status=status.HTTP_201_CREATED)
    return Response(serializer.errors, status=status.HTTP_400_BAD_REQUEST)

@api_view(['GET'])
def get_item(request):
    items = Item.objects.all()
    serializer = ItemSerializer(items, many=True)
    return Response(serializer.data, status=status.HTTP_200_OK)

@api_view(['GET', 'PUT', 'DELETE'])
def manage_item(request, pk):
    item = get_object_or_404(Item, pk=pk)

    if request.method == 'GET':
        return Response(ItemSerializer(item).data, status=status.HTTP_200_OK)

    elif request.method == 'PUT':
        serializer = ItemSerializer(item, data=request.data)
        if serializer.is_valid():
            serializer.save()
            return Response(serializer.data, status=status.HTTP_200_OK)
        return Response(serializer.errors, status=status.HTTP_400_BAD_REQUEST)

    elif request.method == 'DELETE':
        Item.delete()
        return Response(status=status.HTTP_204_NO_CONTENT)

@api_view(['GET'])
def filter_items(request):
    item_class = request.query_params.get('item_class')
    item_type = request.query_params.get('item_type')
    if not item_class or not item_type:
        return Response(
            {"detail": "Both 'item_class' and 'item_type' query parameters are required."}, 
            status=status.HTTP_400_BAD_REQUEST
        )
    items = Item.objects.filter(item_class=item_class, item_type=item_type)
    if not items.exists():
        return Response(
            {"detail": "No items found for the specified parameters."}, 
            status=status.HTTP_404_NOT_FOUND
        )
    serializer = ItemSerializer(items, many=True)
    return Response(serializer.data, status=status.HTTP_200_OK)

@api_view(['GET'])
def get_item_classes(request):
    item_class = Item._meta.get_field('item_class')
    item_classes = item_class.choices

    item_classes_data = [{"item_class": item_class} for item_class, _ in item_classes]

    return Response(item_classes_data, status=status.HTTP_200_OK)


@api_view(['GET'])
def get_item_types(request):
    item_type = Item._meta.get_field('item_type')
    item_types = item_type.choices

    item_types_data = [{"item_type": item_type} for item_type, _ in item_types] 

    return Response(item_types_data, status=status.HTTP_200_OK)