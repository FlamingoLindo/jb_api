import logging
from django.db.models import Q
from django.shortcuts import get_object_or_404
from rest_framework.decorators import api_view, permission_classes
from rest_framework.permissions import IsAuthenticated
from rest_framework.response import Response
from rest_framework import status
from rest_framework.exceptions import PermissionDenied
from ..models import CustomUser
from ..serializer import CustomUserSerializer
from ..pagination import CustomPagination

logger = logging.getLogger(__name__)

@api_view(['GET'])
@permission_classes([IsAuthenticated])
def get_users(request):
    users = CustomUser.objects.exclude(pk=request.user.pk)

    paginator = CustomPagination()
    result_page = paginator.paginate_queryset(users, request)
    serializer = CustomUserSerializer(result_page, many=True)

    return paginator.get_paginated_response(serializer.data)

@api_view(['GET', 'PUT', 'DELETE'])
@permission_classes([IsAuthenticated])
def user_detail(request, pk):
    user = get_object_or_404(CustomUser, pk=pk)

    if request.method == 'GET':
        return Response(CustomUserSerializer(user).data)
    
    elif request.method == 'PUT':
        serializer = CustomUserSerializer(user, data=request.data)
        if serializer.is_valid():
            serializer.save()
            return Response(serializer.data)
        return Response(serializer.errors, status=status.HTTP_400_BAD_REQUEST)
    
    elif request.method == 'DELETE':
        user.delete()
        return Response(status=status.HTTP_204_NO_CONTENT)
    
@api_view(['PUT'])
@permission_classes([IsAuthenticated])
def change_user_status(request, pk):
    try:
        user = CustomUser.objects.get(pk=pk)
    except CustomUser.DoesNotExist:
        return Response({'detail': 'Usuário não existe.'}, status=status.HTTP_404_NOT_FOUND)

    user.is_active = not user.is_active
    user.save(update_fields=['is_active'])

    return Response({
        'id': user.id,
        'is_active': user.is_active
    }, status=status.HTTP_200_OK)

@api_view(['GET'])
#@permission_classes([IsAuthenticated])
def search_user(request):

    query = request.query_params.get('q', '').strip()
    if not query:
        return Response(
            {'detail': "Please provide a search term using the 'q' parameter."},
            status=status.HTTP_400_BAD_REQUEST
        )

    # Filter by email or name
    qs = CustomUser.objects.filter(
        Q(email__icontains=query) |
        Q(name__icontains=query)
    ).exclude(pk=request.user.pk)  # optionally exclude yourself

    # Optional: paginate the results
    paginator = CustomPagination()
    page = paginator.paginate_queryset(qs, request)
    serializer = CustomUserSerializer(page, many=True)
    return paginator.get_paginated_response(serializer.data)
