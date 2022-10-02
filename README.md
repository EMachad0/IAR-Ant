# IAR-Ant

Simulation of Ant based clustering using Rust and Bevy.

![showcase.png](assets/img/showcase.png)

### Equação de probabilade

Para cada individuo a probabilidade de pegar ou largar um item foi calculada da seguinte maneira:

![equation.png](assets/img/equation.png)

O gráfico abaixo mostra a relação entre a razão de items visiveis e a probabilidade

![probability_function.png](assets/img/probability_function.png)

### Raio de visão

Foram realizadas três simulações

Uma com raio um (6 células visiveis)
![raio-1.gif](assets/gif/raio-1.gif)

Uma com raio dois (19 células visiveis)
![raio-2.gif](assets/gif/raio-2.gif)

Uma com raio três (31 células visiveis)
![raio-3.gif](assets/gif/raio-3.gif)

