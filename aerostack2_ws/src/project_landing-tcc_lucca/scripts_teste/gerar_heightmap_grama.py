# gerar_heightmap_grama.py (v2 — com rugosidade garantida fora das zonas planas)
import numpy as np
from PIL import Image
from scipy.ndimage import gaussian_filter

tamanho_px = 256
area_m = 30.0
altura_max_m = 0.35        # sobe um pouco pra acomodar as duas camadas
ruido_escala_macro = 8     # "formato" geral do terreno (suave)
ruido_escala_fino = 0.8    # "textura" da grama (quase sem suavizar -- ISSO é o que garante rugosidade)
amplitude_fino = 0.18      # metros -- amplitude da rugosidade fina (crítico: precisa estourar max_roughness=0.10m)

np.random.seed(42)

# --- Camada macro (relevo suave, dá o "formato") ---
ruido_macro = np.random.rand(tamanho_px, tamanho_px)
macro_suave = gaussian_filter(ruido_macro, sigma=ruido_escala_macro/4)
macro_suave = (macro_suave - macro_suave.min()) / (macro_suave.max() - macro_suave.min())

# --- Camada fina (rugosidade real, pouco suavizada -- garante RMS alto em raio pequeno) ---
ruido_fino = np.random.rand(tamanho_px, tamanho_px)
fino_suave = gaussian_filter(ruido_fino, sigma=ruido_escala_fino/4)
fino_suave = (fino_suave - fino_suave.min()) / (fino_suave.max() - fino_suave.min())
fino_suave = (fino_suave - 0.5) * 2  # centraliza em torno de 0 (-1 a +1)

heightmap = (macro_suave * (altura_max_m - amplitude_fino)) + (fino_suave * amplitude_fino)
heightmap = np.clip(heightmap, 0, altura_max_m)

# --- Zonas planas conhecidas (ground truth) -- SEM a camada fina, achatadas de verdade ---
zonas_planas = [
    (40, 70, 40, 70),
    (150, 180, 60, 90),
    (90, 120, 170, 200),
]
for r0, r1, c0, c1 in zonas_planas:
    heightmap[r0:r1, c0:c1] = 0.02

heightmap_16bit = (heightmap / altura_max_m * 65535).astype(np.uint16)
img = Image.fromarray(heightmap_16bit, mode='I;16')
img.save('grama_alta.png')

print(f"Heightmap v2 gerado: {tamanho_px}x{tamanho_px}px, altura max {altura_max_m}m")
print(f"Rugosidade fina: amplitude {amplitude_fino}m, escala {ruido_escala_fino}px "
      f"(~{ruido_escala_fino/tamanho_px*area_m*100:.1f}cm)")
print(f"Zonas planas (ground truth): {zonas_planas}")