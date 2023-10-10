from django.urls import path
from .views import GetIPView

urlpatterns = [
    path('get-ip', GetIPView.as_view(), name='get-ip'),
]
