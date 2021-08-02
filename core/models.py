from django.db import models

class User(models.Model):
    name=models.CharField(max_length=200)
    def __str__(self):
            return self.name


class ZipFile(models.Model):
    name=models.CharField(max_length=200)
    data=models.FileField()

    def __str__(self):
            return self.name
