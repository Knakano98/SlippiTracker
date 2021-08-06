from rest_framework import serializers
from .models import User
from .models import Game

class UserSerializer(serializers.ModelSerializer):

    class Meta:
        model = User
        fields = "__all__"


class GameSerializer(serializers.ModelSerializer):

    class Meta:
        model = Game
        fields = "__all__"
