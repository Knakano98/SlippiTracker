from django.db import models

class User(models.Model):
    name=models.CharField(max_length=200)
    def __str__(self):
            return self.name





#Game[game_id, date, duration, platform, stage, victorCode]
class Game(models.Model):
    date=models.CharField(max_length=200)
    duration=models.CharField(max_length=200)
    platform=models.CharField(max_length=200)
    stage =models.CharField(max_length=200)
    victor= models.CharField(max_length=200)

class PlayerSessionInfo(models.Model):
    character=models.CharField(max_length=200)
    color=models.CharField(max_length=200)
    netplayCode=models.CharField(max_length=200)
    name=models.CharField(max_length=200)
    port= models.CharField(max_length=200)
    game=models.ForeignKey(Game,on_delete=models.CASCADE)
