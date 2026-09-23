# ##gera tres areas planas iguais

# # de rugosidade entre zonas planas e rugosas.
# import numpy as np
# from PIL import Image
# from scipy.ndimage import gaussian_filter

# tamanho_px = 256
# area_m = 15.0   # tamanho real usado no SDF (<size>15 15 0.3</size>)
# altura_max_m = 0.35
# ruido_escala_macro = 8
# ruido_escala_fino = 6.0    # ERA 0.8 -- aumentado ~7.5x (comprimento de onda maior)
# amplitude_fino = 0.18      # MANTIDA igual a v2, pra isolar so a escala

# np.random.seed(42)

# ruido_macro = np.random.rand(tamanho_px, tamanho_px)
# macro_suave = gaussian_filter(ruido_macro, sigma=ruido_escala_macro/4)
# macro_suave = (macro_suave - macro_suave.min()) / (macro_suave.max() - macro_suave.min())

# ruido_fino = np.random.rand(tamanho_px, tamanho_px)
# fino_suave = gaussian_filter(ruido_fino, sigma=ruido_escala_fino/4)
# fino_suave = (fino_suave - fino_suave.min()) / (fino_suave.max() - fino_suave.min())
# fino_suave = (fino_suave - 0.5) * 2

# heightmap = (macro_suave * (altura_max_m - amplitude_fino)) + (fino_suave * amplitude_fino)
# heightmap = np.clip(heightmap, 0, altura_max_m)

# # MESMAS zonas planas, MESMAS posicoes -- so isso garante comparabilidade
# zonas_planas = [
#     (40, 70, 40, 70),
#     (150, 180, 60, 90),
#     (90, 120, 170, 200),
# ]
# for r0, r1, c0, c1 in zonas_planas:
#     heightmap[r0:r1, c0:c1] = 0.02

# heightmap_16bit = (heightmap / altura_max_m * 65535).astype(np.uint16)
# img = Image.fromarray(heightmap_16bit, mode='I;16')
# img.save('grama_alta_grossa.png')

# escala_cm = ruido_escala_fino / tamanho_px * area_m * 100
# print(f"Heightmap v3 (rugosidade grossa) gerado: {tamanho_px}x{tamanho_px}px")
# print(f"Rugosidade fina: amplitude {amplitude_fino}m, comprimento de onda ~{escala_cm:.1f}cm "
#       f"(era ~{0.8/tamanho_px*area_m*100:.1f}cm na v2)")
# print(f"Zonas planas (mesmas posicoes da v2): {zonas_planas}")

# #de rugosidade entre zonas planas e rugosas.

# # v4: substitui as 3 zonas "iguais" da v3 por 3 zonas DIFERENCIADAS,
# # para testar se o pipeline de percepcao realmente discrimina qualidade
# # entre candidatos (nao so aceita/rejeita, mas tambem pondera raio,
# # rugosidade e distancia de forma sensata).

# #   Zona A: grande, perfeitamente lisa, LONGE do ponto de decolagem (9,0)
# #   Zona B: pequena, levemente rugosa, PERTO do ponto de decolagem (9,0)
# #   Zona C: rampa inclinada (~19 graus) -- deve ser REJEITADA pelo filtro
# #           geometrico (max_inclination_deg=15 no candidate_generation.py)


## gera tres areas diferentes entr si
import numpy as np
from PIL import Image
from scipy.ndimage import gaussian_filter

tamanho_px = 256
area_m = 15.0   # tamanho real usado no SDF (<size>15 15 0.3</size>)
altura_max_m = 0.35
ruido_escala_macro = 8
ruido_escala_fino = 6.0
amplitude_fino = 0.18

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

# --- Conversao pixel <-> mundo (mesma convencao ja usada no candidate_generation.py) ---
pixel_size = area_m / tamanho_px  # metros por pixel

def mundo_para_pixel(x_m, y_m, size_m):
    """Retorna (row0, row1, col0, col1) em pixels para um patch quadrado
    centrado em (x_m, y_m) com lado size_m, na convencao:
        x_mundo = (col - 128) * pixel_size
        y_mundo = -(row - 128) * pixel_size
    """
    col_centro = 128 + x_m / pixel_size
    row_centro = 128 - y_m / pixel_size
    half_px = round((size_m / 2) / pixel_size)
    row0, row1 = round(row_centro) - half_px, round(row_centro) + half_px
    col0, col1 = round(col_centro) - half_px, round(col_centro) + half_px
    return row0, row1, col0, col1


# ============================================================
# ZONA A -- grande, lisa, longe (do ponto de decolagem em x=9,y=0)
# ============================================================
zona_a_centro = (-5.5, -5.5)
zona_a_tamanho = 2.5
r0, r1, c0, c1 = mundo_para_pixel(*zona_a_centro, zona_a_tamanho)
heightmap[r0:r1, c0:c1] = 0.02  # plana, altura baixa e constante, zero ruido

# ============================================================
# ZONA B -- pequena, levemente rugosa, perto do ponto de decolagem
# ============================================================
zona_b_centro = (6.0, 0.0)
zona_b_tamanho = 1.0
r0, r1, c0, c1 = mundo_para_pixel(*zona_b_centro, zona_b_tamanho)
n_rows, n_cols = r1 - r0, c1 - c0
ruido_zona_b = np.random.normal(loc=0.0, scale=0.02, size=(n_rows, n_cols))  # +-2cm
heightmap[r0:r1, c0:c1] = 0.02 + ruido_zona_b
heightmap[r0:r1, c0:c1] = np.clip(heightmap[r0:r1, c0:c1], 0, altura_max_m)

# ============================================================
# ZONA C -- rampa inclinada, DEVE SER REJEITADA (~19 graus > limite de 15)
# ============================================================
zona_c_centro = (0.0, -6.0)
zona_c_tamanho = 1.0
r0, r1, c0, c1 = mundo_para_pixel(*zona_c_centro, zona_c_tamanho)
n_rows = r1 - r0
rampa = np.linspace(0.0, 0.35, n_rows).reshape(-1, 1)  # gradiente linear ao longo das linhas
heightmap[r0:r1, c0:c1] = np.tile(rampa, (1, c1 - c0))

angulo_rampa_deg = np.degrees(np.arctan(0.35 / zona_c_tamanho))

heightmap_16bit = (heightmap / altura_max_m * 65535).astype(np.uint16)
img = Image.fromarray(heightmap_16bit, mode='I;16')
img.save('tres_Areas_diferentes.png')

print(f"Heightmap v4 (zonas diferenciadas) gerado: {tamanho_px}x{tamanho_px}px")
print(f"Zona A (grande/lisa/longe):  centro={zona_a_centro}m, tamanho={zona_a_tamanho}m")
print(f"Zona B (pequena/rugosa/perto): centro={zona_b_centro}m, tamanho={zona_b_tamanho}m")
print(f"Zona C (rampa, deve rejeitar): centro={zona_c_centro}m, tamanho={zona_c_tamanho}m, "
      f"inclinacao ~{angulo_rampa_deg:.1f} graus (limite do filtro: 15 graus)")