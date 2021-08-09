from django.shortcuts import render
from rest_framework.response import Response
from rest_framework.decorators import api_view
from rest_framework import status
from django.http import FileResponse
from .models import *
from .serializers import *
from slippi import Game as slippiGame #https://github.com/hohav/py-slippi
from zipfile import ZipFile
import os
from django.core.files.storage import FileSystemStorage
from django.db import connection
from django.http import HttpResponse
import json
import mylib
def home(request):
    return render(request,'index.html')



@api_view(['POST'])
def zip_upload(request):
    #Get zip file from react, unzip and process slippi files into DBMS.
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
            gameArray.append(slippiGame(i))

        #Game[game_id, date, duration, platform, stage, victorCode]
        #game_id is unique key, victorCode being netplay code of winner

        # playerSessionInfo[playerSessionInfoID,character, color, netplayCode,name, port, game_id]
        # playerSessionInfoID is unqiueKey, game_id foreign key referencing Game

        for i in gameArray:
            dateInput = i.metadata.date
            durationInput= i.metadata.duration
            platformInput= i.metadata.platform
            stageInput= i.start.stage
            #To get victor, find at the last frame in game which player has the 0 or 1 stocks (Or 1 to account for ragequits, not 2 or more since most likely resets)

            player0Stocks= i.frames[durationInput-1].ports[0].leader.post.stocks
            player1Stocks=i.frames[durationInput-1].ports[1].leader.post.stocks


            victorInput=""
            #NOTE: Below code may break when not played on port 1 and 2, although I THINK Slippi auto places players into ports 1 and 2, I maybe wrong

            #print(player0Stocks)
            #print(player1Stocks)

            if player0Stocks>player1Stocks:
                victorInput=i.metadata.players[0].netplay.code
            else:
                victorInput=i.metadata.players[1].netplay.code


            #Create new entry in Game table and insert
            game=Game(date=dateInput,duration=durationInput,platform=platformInput,stage=stageInput,victor=victorInput)
            game.save()

            #print(victor)
            #print(i.frames[durationInput-1].ports[1].leader.post.stocks)
            game_id=game.id

            char0= i.start.players[0].character
            color0= i.start.players[0].costume
            port0 = 0  #May be redundant, will store port though just in case
            netplayCode0=i.metadata.players[0].netplay.code
            name0= i.metadata.players[0].netplay.name

            #Create new entry in PlayerSessionInfo table and insert appropriate session info
            playerSessionInfo=PlayerSessionInfo(character=char0,color=color0,netplayCode=netplayCode0,name=name0,port=port0,game=game)
            playerSessionInfo.save()

            char1= i.start.players[1].character
            color1= i.start.players[1].costume
            port1 = 1  #May be redundant, will store port though just in case
            netplayCode1=i.metadata.players[1].netplay.code
            name1= i.metadata.players[1].netplay.name

            playerSessionInfo=PlayerSessionInfo(character=char1,color=color1,netplayCode=netplayCode1,name=name1,port=port1,game=game)
            playerSessionInfo.save()

        print(Game.objects.values())
        print(PlayerSessionInfo.objects.values())


        print('Done!')

    return Response(status=status.HTTP_201_CREATED)


@api_view(['GET'])
def game_list(request):
    data = Game.objects.all()

    serializer = GameSerializer(data, context={'request': request}, many=True)

    return Response(serializer.data)


@api_view(['GET'])
def get_user_info(request):
    code=request.query_params.get("code")
    #print(str(code))
    fixedCode=""

    for i in str(code):
        if(i=='_'):
            fixedCode=fixedCode+'#'
        else:
            fixedCode=fixedCode+i


    print(fixedCode)

    #Join core_game with core_playersessioninfo
    #get winrate
    #get games where player played in
    #Check in those games who the victor was
    #playerSession = PlayerSessionInfo.objects.select_related( 'game').filter(netplayCode=fixedCode)
    playerSession = PlayerSessionInfo.objects.select_related( 'game').filter(netplayCode=fixedCode)
    total=0
    wins=0


    print(playerSession)

    #playerSession contains a querySet
    #Convert querySet to a form that can be passed to rust (Json?)
    #Import rust function that takes in tghe json, then return the relevent info
    #Process data in rust, then return values


    #Convert this section to rust
    for i in playerSession:
        total=total+1
        if(i.game.victor==i.netplayCode):
            wins=wins+1
        #print(i.game.victor)
        #print(i.netplayCode)
    #innerjoin core_game with core_playersessinfo with the core_playersessinfo.game_id and  core_game.id




    print("wins " + str(wins))
    print("total " + str(total))

    winRate= wins/total*100

    print(winRate)

    data={"winRate": winRate, "total": total}

    print(data)
    return Response(data)


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
