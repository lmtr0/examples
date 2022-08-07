
from django.urls import path, include
from . import views

urlpatterns = [
    path('', views.index),
    path('test/<int:year>/', views.with_args),
]