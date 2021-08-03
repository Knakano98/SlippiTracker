from django.shortcuts import render
from rest_framework.response import Response
from rest_framework.decorators import api_view
from rest_framework import status
from django.http import FileResponse
from .models import *
from .serializers import *
from slippi import Game
from zipfile import ZipFile
import os
from django.core.files.storage import FileSystemStorage

def home(request):
    return render(request,'index.html')

@api_view(['POST'])
def zip_upload(request):
    print("POST ZIP FILE")

    print(request.FILES);


    zipFile = request.FILES['file']
    #print(zipFile.name)
    #print(zipFile.size)
    gameArray=[]


    with ZipFile(zipFile, 'r') as zip:
        # printing all the contents of the zip file
        fileList= zip.namelist()

        #zip.printdir()
        #Store file names in zipfile, then use that after .extractall()

        # extracting all the files
        print('Extracting all the files now...')
        #process slippi files here

        zip.extractall()


        for i in fileList:
            gameArray.append(Game(i))


        for i in gameArray:
            print(i)

        print('Done!')

    return Response(status=status.HTTP_201_CREATED)


# @api_view(['GET', 'POST'])
# def user_list(request):
#     if request.method == 'GET':
#         data = User.objects.all()
#
#         serializer = UserSerializer(data, context={'request': request}, many=True)
#
#         return Response(serializer.data)
#
#     elif request.method == 'POST':
#         print("POST" + str(request.data))
#         serializer = UserSerializer(data=request.data)
#         if serializer.is_valid():
#             serializer.save()
#             return Response(status=status.HTTP_201_CREATED)
#
#         return Response(serializer.errors, status=status.HTTP_400_BAD_REQUEST)




# @api_view(['PUT', 'DELETE'])
# def user_detail(request, pk):
#     try:
#         user = User.objects.get(pk=pk)
#     except User.DoesNotExist:
#         return Response(status=status.HTTP_404_NOT_FOUND)
#
#     if request.method == 'PUT':
#         serializer = UserSerializer(user, data=request.data,context={'request': request})
#         if serializer.is_valid():
#             serializer.save()
#             return Response(status=status.HTTP_204_NO_CONTENT)
#         return Response(serializer.errors, status=status.HTTP_400_BAD_REQUEST)
#
#     elif request.method == 'DELETE':
#         user.delete()
#         return Response(status=status.HTTP_204_NO_CONTENT)
