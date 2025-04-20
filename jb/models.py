from django.db import models
from django.contrib.auth.models import AbstractBaseUser, PermissionsMixin
from django.core.validators import MinLengthValidator
from django.contrib.auth.models import (
    AbstractBaseUser, PermissionsMixin,
    BaseUserManager, Group, Permission
)

# Create your models here.

class Brand(models.Model):
    name = models.CharField(max_length=100, null=False, blank=False, unique=True)
    logo = models.ImageField(upload_to='Brands', null=True, blank=True)

    createdAt = models.DateTimeField(auto_now_add=True)
    updatedAt = models.DateTimeField(auto_now=True)

    def __str__(self):
        return self.name

    class Meta:
        ordering = ['name']
        verbose_name = 'Brand'
        verbose_name_plural = 'Brands'

class Product(models.Model):
    brand = models.ForeignKey(
        Brand, on_delete=models.CASCADE, related_name='Products', null=True, blank=True
    )
    name = models.CharField(max_length=200, null=False, blank=False)
    code = models.CharField(max_length=200, null=False, blank=False, unique=True)
    description = models.TextField(null=True, blank=True)
    price = models.DecimalField(max_digits=10, decimal_places=2, null=False, blank=False)
    original_price = models.DecimalField(max_digits=10, decimal_places=2, null=False, blank=False, default=0.00)
    image = models.ImageField(upload_to='Products', null=True, blank=True)

    createdAt = models.DateTimeField(auto_now_add=True)
    updatedAt = models.DateTimeField(auto_now=True)

    def save(self, *args, **kwargs):
        if not self.pk:
            self.original_price = self.price
        super().save(*args, **kwargs)

    def __str__(self):
        return self.name

    class Meta:
        ordering = ['name']
        verbose_name = 'Product'
        verbose_name_plural = 'Products'

class ItemClasses(models.TextChoices):
    CLASS1 = '05-FERROS TREFILADOS 1020', '05-FERROS TREFILADOS 1020'
    CLASS2 = '06-FERROS TREFILADOS 1045', '06-FERROS TREFILADOS 1045'
    CLASS3 = '33-8620-8640', '33-8620-8640'
    CLASS4 = '34-4140-4340', '34-4140-4340'
    CLASS5 = '30-AÇO INOX', '30-AÇO INOX'
    CLASS6 = '25-FERRO REDONDO MECÂNICO', '25-FERRO REDONDO MECÂNICO'
    CLASS7 = '29-VC-131', '29-VC-131'
    CLASS8 = '35-VND', '35-VND'
    CLASS9 = '28-AÇO PRATA', '28-AÇO PRATA'
    CLASS10 = '08-AÇO 1112', '08-AÇO 1112'
    CLASS11 = '21-FERROS CHATOS', '21-FERROS CHATOS'
    CLASS12 = '22-FERROS CANTONEIRAS', '22-FERROS CANTONEIRAS'
    CLASS13 = '24-FERRO QUADRADO', '24-FERRO QUADRADO'
    CLASS14 = '23-FERRO TEE', '23-FERRO TEE'
    CLASS15 = '37-F.FUNDIDO', '37-F.FUNDIDO'
    CLASS16 = '26-PLÁSTICOS DE ENGENHARIA', '26-PLÁSTICOS DE ENGENHARIA'
    CLASS17 = '27-TUBOS MECÂNICOS', '27-TUBOS MECÂNICOS'
    CLASS18 = '20-LATÃO', '20-LATÃO'
    CLASS19 = '36-BRONZE TM 23-620', '36-BRONZE TM 23-620'
    CLASS20 = '19-BUCHAS DE BRONZE TM 23', '19-BUCHAS DE BRONZE TM 23'
    CLASS21 = '32-ALUMÍNIO', '32-ALUMÍNIO'
    CLASS22 = '31-COBRE', '31-COBRE'
    CLASS23 = '10-CHAPAS CORTADAS', '10-CHAPAS CORTADAS'
    CLASS24 = '11-CHAPAS DOBRADAS', '11-CHAPAS DOBRADAS'
    CLASS25 = '09-VIGAS LAMINADAS', '09-VIGAS LAMINADAS'
    CLASS26 = '12-TUBOS REDONDOS', '12-TUBOS REDONDOS'
    CLASS27 = '13-TUBOS QUADRADOS', '13-TUBOS QUADRADOS'
    CLASS28 = '16-TUBO DIN 2440', '16-TUBO DIN 2440'
    CLASS29 = '17-TUBO SCH 80', '17-TUBO SCH 80'
    CLASS30 = '18-TUBOS TREFILADOS', '18-TUBOS TREFILADOS'
    CLASS31 = '14-TUBOS RETANGULARES', '14-TUBOS RETANGULARES'
    CLASS32 = '15-TUBOS GALVANIZADOS', '15-TUBOS GALVANIZADOS'

class ItemTypes(models.TextChoices):
    TYPE1 = 'REDONDO', 'REDONDO'
    TYPE2 = 'REDONDO MM', 'REDONDO MM'
    TYPE3 = 'SEXTAVADO', 'SEXTAVADO'
    TYPE4 = 'QUADRADO', 'QUADRADO'
    TYPE5 = 'AÇO 4140', 'AÇO 4140'
    TYPE6 = 'AÇO 4340', 'AÇO 4340'
    TYPE7 = 'CANTONEIRA', 'CANTONEIRA'
    TYPE8 = 'CHATO', 'CHATO'
    TYPE9 = 'TUBO REDONDO', 'TUBO REDONDO'
    TYPE10 = 'TUBO QUADRADO', 'TUBO QUADRADO'
    TYPE11 = 'TUBO SCH', 'TUBO SCH'
    TYPE12 = 'TUBO RETANGULAR', 'TUBO RETANGULAR'   
    TYPE13 = 'AÇO 1020', 'AÇO 1020'
    TYPE14 = 'AÇO 1045', 'AÇO 1045'
    TYPE15 = 'COLUNA1', 'COLUNA1'
    TYPE16 = 'TEE', 'TEE'
    TYPE17 = 'REDONDO CINZENDO', 'REDONDO CINZENDO'
    TYPE18 = 'REDONDO NODULAR', 'REDONDO NODULAR'
    TYPE19 = 'QUADRADO CINZENTO', 'QUADRADO CINZENTO'
    TYPE20 = 'RETANGULAR CINZENTO', 'RETANGULAR CINZENTO'
    TYPE21 = 'TARUGO DE NYLON 6,0', 'TARUGO DE NYLON 6,0'
    TYPE22 = 'TARUGO DE NYLON 6,0 QUADRADO', 'TARUGO DE NYLON 6,0 QUADRADO'
    TYPE23 = 'TARUGO DE NYLON TECHNYL 6.6', 'TARUGO DE NYLON TECHNYL 6.6'
    TYPE24 = 'TARUGO PP  (POLIPROPILENO)', 'TARUGO PP  (POLIPROPILENO)'
    TYPE25 = 'TARUGO DE POLIACETAL(POM)', 'TARUGO DE POLIACETAL(POM)'
    TYPE26 = 'TARUGO UHMW', 'TARUGO UHMW'
    TYPE27 = 'TEFLON', 'TEFLON'
    TYPE28 = 'TARUGO CELERON', 'TARUGO CELERON'
    TYPE29 = 'TM 23', 'TM 23'
    TYPE30 = 'TM 620', 'TM 620'
    TYPE31 = 'BUCHA', 'BUCHA'
    TYPE32 = 'TUBO', 'TUBO'
    TYPE33 = 'CORTADAS', 'CORTADAS'
    TYPE34 = 'DOBRADAS', 'DOBRADAS'
    TYPE35 = 'VIGA U', 'VIGA U'
    TYPE36 = 'VIGA I', 'VIGA I'
    TYPE37 = 'VIGA PERFIL I', 'VIGA PERFIL I'
    TYPE38 = 'VIGA PERFIL H', 'VIGA PERFIL H'
    TYPE39 = 'TUBO DIN', 'TUBO DIN'
    TYPE40 = 'SCH40', 'SCH40'
    TYPE41 = 'SCH80', 'SCH80'
    TYPE42 = 'TUBO GALVANIZADO', 'TUBO GALVANIZADO'
    TYPE43 = 'TUBO TREFILADO', 'TUBO TREFILADO'

class Item(models.Model):

    item_class = models.CharField(
        max_length=200,
        choices=ItemClasses.choices,
    )

    item_type = models.CharField(
        max_length=200,
        choices=ItemTypes.choices,
    )

    item_name = models.CharField(
        max_length=200,
        null=False,
        blank=False
    )

    weight_mt = models.DecimalField(
        max_digits=10,
        decimal_places=3,
        null=True,
        blank=True
    )

    weight_kg_no_cut = models.DecimalField(
        max_digits=10,
        decimal_places=2,
        null=True,
        blank=True
    )

    value_meter = models.DecimalField(
        max_digits=10,
        decimal_places=2,
        null=True,
        blank=True
    )

    cut_percent = models.IntegerField(
        null=True,
        blank=True
    )

    value_kg_w_cut = models.DecimalField(
        max_digits=10,
        decimal_places=2,
        null=False,
        blank=False
    )

    parts_amount = models.IntegerField(
        null=True,
        blank=True
    )

    weight_mm = models.DecimalField(
        max_digits=10,
        decimal_places=8,
        null=True,
        blank=True
    )

    length_mm = models.IntegerField(
        null=True,
        blank=True
    )

    value_w_cut = models.DecimalField(
        max_digits=10,
        decimal_places=2,
        null=True,
        blank=True
    )

    weight_kg = models.DecimalField(
        max_digits=10,
        decimal_places=2,
        null=True,
        blank=True
    )

    value_no_cut = models.DecimalField(
        max_digits=10,
        decimal_places=2,
        null=True,
        blank=True
    )

    createdAt = models.DateTimeField(auto_now_add=True)
    updatedAt = models.DateTimeField(auto_now=True)

    def __str__(self):
        return self.item_name

    class Meta:
        ordering = ['item_name']
        verbose_name = 'Item'
        verbose_name_plural = 'Items'

class CustomUserManager(BaseUserManager):
    def get_by_natural_key(self, email):
        return self.get(email=email)

    def create_user(self, email, password=None, **extra_fields):
        if not email:
            raise ValueError('The Email field must be set')
        email = self.normalize_email(email)
        user = self.model(email=email, **extra_fields)
        user.set_password(password)
        user.save(using=self._db)
        return user

    def create_superuser(self, email, password=None, **extra_fields):
        extra_fields.setdefault('is_staff', True)
        extra_fields.setdefault('is_superuser', True)
        return self.create_user(email, password, **extra_fields)

class CustomUser(AbstractBaseUser, PermissionsMixin):
    name = models.CharField(
        max_length=100,
        validators=[MinLengthValidator(3)]
    )
    email = models.EmailField(unique=True)

    password = models.CharField(max_length=128)

    is_staff  = models.BooleanField(default=False)
    is_active = models.BooleanField(default=True)
    createdAt = models.DateTimeField(auto_now_add=True)
    updatedAt = models.DateTimeField(auto_now=True)

    objects = CustomUserManager()

    USERNAME_FIELD = "email"
    REQUIRED_FIELDS = ["name"]

    # ** Override the clashes here: **
    groups = models.ManyToManyField(
        Group,
        verbose_name=("groups"),
        blank=True,
        related_name="customuser_groups",      # ← unique name
        related_query_name="customuser_group", # optional
        help_text=(
            "The groups this user belongs to. Permissions are "
            "inherited from each of their groups."
        ),
    )
    user_permissions = models.ManyToManyField(
        Permission,
        verbose_name=("user permissions"),
        blank=True,
        related_name="customuser_permissions",      # ← unique name
        related_query_name="customuser_permission", # optional
        help_text=(
            "Specific permissions for this user."
        ),
    )

    class Meta:
        ordering = ["-createdAt"]
        verbose_name = "User"
        verbose_name_plural = "Users"

    def __str__(self):
        return self.name