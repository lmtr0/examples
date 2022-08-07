from django.shortcuts import render
from django.http import HttpRequest, HttpResponse
from .models import Question

def index(request):
    latest_question_list = Question.objects.order_by('-pub_date')[:5]
    context = {'latest_question_list': latest_question_list}
    return render(request, 'polls/index.html', context)
    

def with_args(request: HttpRequest, name):
    return HttpResponse(f'hello {name}')