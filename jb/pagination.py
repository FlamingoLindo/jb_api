from rest_framework.pagination import PageNumberPagination

# Only lists 25 items per page
# http://127.0.0.1:8000/natour_api/users/?page=2
# Even if the user requests more than 25 items, "?page_size=100" will not work
class CustomPagination(PageNumberPagination):
    page_size = 25
    page_size_query_param = 'page_size'
    max_page_size = 100
