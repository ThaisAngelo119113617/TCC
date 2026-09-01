# # gerar_heightmap_grama.py (v2 — com rugosidade garantida fora das zonas planas)
# import numpy as np
# from PIL import Image
# from scipy.ndimage import gaussian_filter

# tamanho_px = 256
# area_m = 30.0
# altura_max_m = 0.35        # sobe um pouco pra acomodar as duas camadas
# ruido_escala_macro = 8     # "formato" geral do terreno (suave)
# ruido_escala_fino = 0.8    # "textura" da grama (quase sem suavizar -- ISSO é o que garante rugosidade)
# amplitude_fino = 0.18      # metros -- amplitude da rugosidade fina (crítico: precisa estourar max_roughness=0.10m)

# np.random.seed(42)

# # --- Camada macro (relevo suave, dá o "formato") ---
# ruido_macro = np.random.rand(tamanho_px, tamanho_px)
# macro_suave = gaussian_filter(ruido_macro, sigma=ruido_escala_macro/4)
# macro_suave = (macro_suave - macro_suave.min()) / (macro_suave.max() - macro_suave.min())

# # --- Camada fina (rugosidade real, pouco suavizada -- garante RMS alto em raio pequeno) ---
# ruido_fino = np.random.rand(tamanho_px, tamanho_px)
# fino_suave = gaussian_filter(ruido_fino, sigma=ruido_escala_fino/4)
# fino_suave = (fino_suave - fino_suave.min()) / (fino_suave.max() - fino_suave.min())
# fino_suave = (fino_suave - 0.5) * 2  # centraliza em torno de 0 (-1 a +1)

# heightmap = (macro_suave * (altura_max_m - amplitude_fino)) + (fino_suave * amplitude_fino)
# heightmap = np.clip(heightmap, 0, altura_max_m)

# # --- Zonas planas conhecidas (ground truth) -- SEM a camada fina, achatadas de verdade ---
# zonas_planas = [
#     (40, 70, 40, 70),
#     (150, 180, 60, 90),
#     (90, 120, 170, 200),
# ]
# for r0, r1, c0, c1 in zonas_planas:
#     heightmap[r0:r1, c0:c1] = 0.02

# heightmap_16bit = (heightmap / altura_max_m * 65535).astype(np.uint16)
# img = Image.fromarray(heightmap_16bit, mode='I;16')
# img.save('grama_alta.png')

# print(f"Heightmap v2 gerado: {tamanho_px}x{tamanho_px}px, altura max {altura_max_m}m")
# print(f"Rugosidade fina: amplitude {amplitude_fino}m, escala {ruido_escala_fino}px "
#       f"(~{ruido_escala_fino/tamanho_px*area_m*100:.1f}cm)")
# print(f"Zonas planas (ground truth): {zonas_planas}")

# gerar_heightmap_grama_v3_grosso.py
# Mesma estrutura da v2, so aumenta a ESCALA (comprimento de onda) da
# rugosidade fina -- mantem amplitude igual, pra isolar essa unica variavel
# e testar se e a escala espacial (nao o voxel) que explica a sobreposicao
# de rugosidade entre zonas planas e rugosas.
import numpy as np
from PIL import Image
from scipy.ndimage import gaussian_filter

tamanho_px = 256
area_m = 15.0   # tamanho real usado no SDF (<size>15 15 0.3</size>)
altura_max_m = 0.35
ruido_escala_macro = 8
ruido_escala_fino = 6.0    # ERA 0.8 -- aumentado ~7.5x (comprimento de onda maior)
amplitude_fino = 0.18      # MANTIDA igual a v2, pra isolar so a escala

np.random.seed(42)

ruido_macro = np.random.rand(tamanho_px, tamanho_px)
macro_suave = gaussian_filter(ruido_macro, sigma=ruido_escala_macro/4)
macro_suave = (macro_suave - macro_suave.min()) / (macro_suave.max() - macro_suave.min())

ruido_fino = np.random.rand(tamanho_px, tamanho_px)
fino_suave = gaussian_filter(ruido_fino, sigma=ruido_escala_fino/4)
fino_suave = (fino_suave - fino_suave.min()) / (fino_suave.max() - fino_suave.min())
fino_suave = (fino_suave - 0.5) * 2

heightmap = (macro_suave * (altura_max_m - amplitude_fino)) + (fino_suave * amplitude_fino)
heightmap = np.clip(heightmap, 0, altura_max_m)

# MESMAS zonas planas, MESMAS posicoes -- so isso garante comparabilidade
zonas_planas = [
    (40, 70, 40, 70),
    (150, 180, 60, 90),
    (90, 120, 170, 200),
]
for r0, r1, c0, c1 in zonas_planas:
    heightmap[r0:r1, c0:c1] = 0.02

heightmap_16bit = (heightmap / altura_max_m * 65535).astype(np.uint16)
img = Image.fromarray(heightmap_16bit, mode='I;16')
img.save('grama_alta_grossa.png')

escala_cm = ruido_escala_fino / tamanho_px * area_m * 100
print(f"Heightmap v3 (rugosidade grossa) gerado: {tamanho_px}x{tamanho_px}px")
print(f"Rugosidade fina: amplitude {amplitude_fino}m, comprimento de onda ~{escala_cm:.1f}cm "
      f"(era ~{0.8/tamanho_px*area_m*100:.1f}cm na v2)")
print(f"Zonas planas (mesmas posicoes da v2): {zonas_planas}")