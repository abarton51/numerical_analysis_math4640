import numpy as np
import matplotlib.pyplot as plt

f = lambda x: np.sin(5*x)**2

N = 200
x_k = np.linspace(0, 2*np.pi, N, endpoint=False)
f_k = f(x_k)

f_hat = np.fft.fft(f_k) / N

j = np.fft.fftfreq(N, d=(2*np.pi/N))

plt.figure(figsize=(8, 5))
plt.stem(j, np.abs(f_hat))
plt.xlabel("Frequency Index j")
plt.ylabel("Magnitude of Fourier Coefficient")
plt.title("Fourier Coefficients of $sin^2(5x)$")
plt.xlim(-10, 10)
plt.grid()
plt.show()

