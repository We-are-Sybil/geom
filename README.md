# Geom: Procesador de Datos Geométricos

## Descripción General

Geom es una potente herramienta de procesamiento de datos diseñada para analizar y transformar datos de coordenadas geográficas manteniendo la confidencialidad de la información. Es especialmente útil para organizaciones que necesitan procesar datos de ubicación sensibles sin comprometer la privacidad.

## Características

- **Transformación de Coordenadas**: Aplica un enfoque de ventana deslizante para transformar coordenadas, preservando el posicionamiento relativo mientras oculta las ubicaciones absolutas.
- **Análisis Temporal**: Convierte la información de fechas en datos angulares para el análisis de patrones cíclicos.
- **Hash de Acciones**: Implementa un mecanismo de hash seguro para datos categóricos, permitiendo el análisis de patrones sin revelar acciones específicas.
- **Procesamiento por Lotes**: Procesa datos en tamaños de lote configurables para un manejo eficiente de grandes conjuntos de datos.
- **Recuperación de Datos de Elevación**: Se integra con APIs externas para obtener datos de elevación para coordenadas dadas.
- **Manejo Flexible de Entrada**: Admite nomenclatura personalizada de columnas en archivos CSV de entrada.
- **Salida Segura**: Genera una salida en un formato que mantiene la utilidad de los datos para el análisis mientras preserva la confidencialidad.
- **Interpolación de Datos**: Implementa un proceso de interpolación para mejorar la precisión y detalle de los datos geográficos.

## Interpolación de Datos

Geom incluye una función de interpolación de datos que mejora significativamente la precisión y el detalle de los análisis geográficos:

- **Discretización de Puntos**: Entre cada par de puntos originales, se añaden puntos adicionales (configurable, por defecto 20 puntos).
- **Cálculo de Elevación**: Para cada punto interpolado, se obtiene la elevación correspondiente mediante una API externa.
- **Mejora de Precisión**: Permite obtener una representación más fiel del terreno entre los puntos reportados.
- **Detección de Anomalías**: Ayuda a identificar posibles irregularidades en el movimiento que podrían no ser evidentes con solo los puntos originales.

### Video Comparativo

En la siguiente demostración visual se puede ver el impacto de la interpolación de datos: 

https://github.com/user-attachments/assets/207ecd79-a643-4afe-8fd0-dc273c6acbed

Este video muestra cómo la interpolación mejora la representación de rutas y terrenos, proporcionando insights más precisos.

## Instalación

### Prerrequisitos

- Lenguaje de programación Rust (última versión estable)
- Gestor de paquetes Cargo

### Pasos

1. Clonar el repositorio:
   ```
   git clone https://github.com/We-are-Sybil/geom.git
   cd geom
   ```

2. Compilar el proyecto:
   ```
   cargo build --release
   ```

## Uso

### Descarga del Ejecutable

Descargue el ejecutable más reciente desde la sección de Releases en GitHub:
[https://github.com/We-are-Sybil/geom/releases](https://github.com/We-are-Sybil/geom/releases)

### Ejecución

Una vez descargado, ejecute el binario con el siguiente comando:

```
./geom -i <archivo_entrada> -o <archivo_salida> -s <salt> -n <puntos_interpolacion>
```

### Argumentos de Línea de Comando

- `-i, --input <ARCHIVO>`: Ruta del archivo CSV de entrada (requerido)
- `-o, --output <ARCHIVO>`: Ruta del archivo de salida (por defecto: "output.csv")
- `-s, --salt <CADENA>`: Salt para el hash (requerido)
- `-d, --date-column <CADENA>`: Nombre de la columna de fecha (por defecto: "Fecha")
- `-a, --action-column <CADENA>`: Nombre de la columna de acción (por defecto: "Accion")
- `-x, --latitude-column <CADENA>`: Nombre de la columna de latitud (por defecto: "Latitud")
- `-y, --longitude-column <CADENA>`: Nombre de la columna de longitud (por defecto: "Longitud")
- `-H, --host <CADENA>`: Host para la API de elevación (por defecto: "api.open-elevation.com")
- `-f, --output-format <FORMATO>`: Formato de salida (por defecto: csv)
- `-n, --num-discretize <NÚMERO>`: Número de puntos a interpolar entre cada par de puntos originales (por defecto: 20)

## Formato del Archivo de Entrada

La entrada debe ser un archivo CSV con columnas para fecha, acción, latitud y longitud. Los nombres de las columnas se pueden especificar usando los argumentos de línea de comando.

## Salida

El programa genera un archivo CSV que contiene:
- ID de lote
- Ángulo transformado (a partir de la fecha)
- Acción hasheada
- Coordenadas transformadas (longitud, latitud, elevación)
- Indicador de punto original o interpolado

Esta salida preserva el posicionamiento relativo y los patrones en los datos mientras oculta la información sensible original.

## Notas Importantes

- Asegúrese de mantener la consistencia en el etiquetado de acciones en el CSV de entrada. El proceso de hash es sensible a variaciones menores en el texto.
- El salt utilizado para el hash debe mantenerse confidencial para preservar la seguridad de los datos transformados.
- Esta herramienta está diseñada para la anonimización de datos y el análisis de patrones. No debe utilizarse como el único método para asegurar información altamente sensible.

## Contribuciones

Las contribuciones a Geom son bienvenidas. Por favor, asegúrese de que su código se adhiere a los estándares de codificación del proyecto e incluya pruebas apropiadas para las nuevas características.

## Licencia

Geom se publica bajo la Licencia Pública de Mozilla 2.0 (MPL-2.0). Esta licencia permite usar, modificar y distribuir el software, y requiere que cualquier modificación a los archivos originales se publique bajo la misma licencia. Sin embargo, se puede combinar el código licenciado bajo MPL-2.0 con código propietario.

Para el texto completo de la licencia, consulte el archivo [LICENSE.md](LICENSE.md) en el directorio raíz de este árbol de fuentes o visite [Mozilla Public License 2.0](https://www.mozilla.org/en-US/MPL/2.0/).
