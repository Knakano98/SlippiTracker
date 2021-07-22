from django.db import migrations

def create_data(apps, schema_editor):
    Student = apps.get_model('core', 'User')
    Student(name="Fpx McCloud").save()

class Migration(migrations.Migration):

    dependencies = [
        ('core', '0001_initial'),
    ]

    operations = [
    	migrations.RunPython(create_data),
    ]
