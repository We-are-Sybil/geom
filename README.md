# Geom: Procesador de Datos Geométricos

## Descripción General

Geom es una potente herramienta de procesamiento de datos diseñada para analizar y transformar datos de coordenadas geográficas manteniendo la confidencialidad de la información. Es especialmente útil para organizaciones que necesitan procesar datos de ubicación sensibles sin comprometer la privacidad.

## Características

- **Transformación de Coordenadas**: Aplica un enfoque de ventana deslizante para transformar coordenadas, preservando el posicionamiento relativo mientras oculta las ubicaciones absolutas.
- **Análisis Temporal**: Convierte la información de fechas en datos angulares para el análisis de patrones cíclicos.
- **Hash de Datos Categóricos**: Implementa un mecanismo de hash seguro para datos categóricos (Batallón, Pelotón, Compañía), permitiendo el análisis de patrones sin revelar información específica.
- **Interpolación de Puntos**: Genera puntos adicionales entre cada par de puntos originales, asignándoles los datos categóricos del primer punto del par.
- **Procesamiento por Lotes**: Procesa datos en tamaños de lote configurables para un manejo eficiente de grandes conjuntos de datos.
- **Recuperación de Datos de Elevación**: Se integra con APIs externas para obtener datos de elevación para coordenadas dadas.
- **Manejo Flexible de Entrada**: Admite nomenclatura personalizada de columnas en archivos CSV de entrada.
- **Salida Segura**: Genera una salida en un formato que mantiene la utilidad de los datos para el análisis mientras preserva la confidencialidad.
- **Mapeo de Valores Originales**: Genera un archivo de mapeo separado que permite a los propietarios de los datos recuperar los valores originales de los datos categóricos.

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

Ejecute el binario compilado con el siguiente comando:

```
./target/release/geom -i <archivo_entrada> -o <archivo_salida> -s <salt> -m <archivo_mapeo>
```

### Argumentos de Línea de Comando

- `-i, --input <ARCHIVO>`: Ruta del archivo CSV de entrada (requerido)
- `-o, --output <ARCHIVO>`: Ruta del archivo de salida (por defecto: "output.csv")
- `-s, --salt <CADENA>`: Salt para el hash (requerido)
- `-d, --date-column <CADENA>`: Nombre de la columna de fecha (por defecto: "Fecha")
- `-b, --battalion-column <CADENA>`: Nombre de la columna de batallón (por defecto: "Batallon")
- `-p, --platoon-column <CADENA>`: Nombre de la columna de pelotón (por defecto: "Peloton")
- `-c, --company-column <CADENA>`: Nombre de la columna de compañía (por defecto: "Compañia")
- `-x, --latitude-column <CADENA>`: Nombre de la columna de latitud (por defecto: "Latitud")
- `-y, --longitude-column <CADENA>`: Nombre de la columna de longitud (por defecto: "Longitud")
- `-H, --host <CADENA>`: Host para la API de elevación (por defecto: "api.open-elevation.com")
- `-f, --output-format <FORMATO>`: Formato de salida (por defecto: csv)
- `-m, --mapping-output <ARCHIVO>`: Ruta del archivo de mapeo de valores originales a hasheados (por defecto: "mapping.csv")
- `-n, --num-discretize <NÚMERO>`: Número de puntos a añadir entre cada par de puntos originales (por defecto: 20)
- `-D, --debug-verbose`: Habilita la depuración detallada para todas las solicitudes
- `-e, --debug-on-error`: Habilita la depuración solo para errores

## Formato del Archivo de Entrada

La entrada debe ser un archivo CSV con columnas para fecha, batallón, pelotón, compañía, latitud y longitud. Los nombres de las columnas se pueden especificar usando los argumentos de línea de comando.

## Salida

El programa genera dos archivos:

1. Un archivo CSV de salida que contiene:
   - ID de lote
   - Ángulo transformado (a partir de la fecha)
   - Batallón hasheado
   - Pelotón hasheado
   - Compañía hasheada
   - Coordenadas transformadas (longitud, latitud, elevación)
   - Indicador de si el punto es original o interpolado

2. Un archivo CSV de mapeo que contiene:
   - Tipo (Batallón, Pelotón, o Compañía)
   - Valor original
   - Valor hasheado

Esta salida preserva el posicionamiento relativo y los patrones en los datos mientras oculta la información sensible original. El archivo de mapeo permite a los propietarios de los datos recuperar los valores originales cuando sea necesario.

## Notas Importantes

- Asegúrese de mantener la consistencia en el etiquetado de batallones, pelotones y compañías en el CSV de entrada. El proceso de hash es sensible a variaciones menores en el texto.
- El salt utilizado para el hash debe mantenerse confidencial para preservar la seguridad de los datos transformados.
- Los puntos interpolados entre cada par de puntos originales heredan los datos categóricos (batallón, pelotón, compañía) del primer punto del par.
- Esta herramienta está diseñada para la anonimización de datos y el análisis de patrones. No debe utilizarse como el único método para asegurar información altamente sensible.
- El archivo de mapeo debe mantenerse seguro, ya que contiene la información necesaria para revertir el proceso de anonimización de los datos categóricos.

## Contribuciones

Las contribuciones a Geom son bienvenidas. Por favor, asegúrese de que su código se adhiere a los estándares de codificación del proyecto e incluya pruebas apropiadas para las nuevas características.

## Licencia

Geom se publica bajo la Licencia Pública de Mozilla 2.0 (MPL-2.0). Esta licencia permite usar, modificar y distribuir el software, y requiere que cualquier modificación a los archivos originales se publique bajo la misma licencia. Sin embargo, se puede combinar el código licenciado bajo MPL-2.0 con código propietario.

Para el texto completo de la licencia, consulte el archivo [LICENSE.md](LICENSE.md) en el directorio raíz de este árbol de fuentes o visite [Mozilla Public License 2.0](https://www.mozilla.org/en-US/MPL/2.0/).
