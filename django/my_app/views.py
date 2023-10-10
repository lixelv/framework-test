from django.http import JsonResponse
from django.views import View

class GetIPView(View):
    def get(self, request, *args, **kwargs):
        ip = get_client_ip(request)
        return JsonResponse({"data": {'ip': ip}})

def get_client_ip(request):
    x_forwarded_for = request.META.get('HTTP_X_FORWARDED_FOR')
    if x_forwarded_for:
        ip = x_forwarded_for.split(',')[0]
    else:
        ip = request.META.get('REMOTE_ADDR')
    return ip
