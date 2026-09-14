Alunos: 
Natália Possani de Lima RA: 24190450-2
Rayan de Andrade e Andrade RA: 24211898-2
Vinicius Colombo RA: 24160033-2

2. Plankalkül não foi implementada em sua época. Ainda assim, por que ela é relevante para a história das linguagens? Cite três recursos antecipados por seu projeto e explique o valor de um deles.
R: Ela é relevante por ser a primeira linguagem de alto nível, trazendo conceitos essenciais da computação moderna antes mesmo de qualquer aplicação prática e comercial. Provando assim, que a lógica poderia existir antes do hardware adequado.
Os recursos trazidos por ela foram: Estruturas de dados avançadas, Aritmética de ponto flutuante e Conceitos de Controle de repetição e sub-rotinas.

4. Explique por que o projeto Fortran precisou convencer programadores de que código traduzido podia competir com código de máquina escrito à mão. Relacione desempenho, custo de programação e adoção.
R: Sobre o desempenho: o projeto enfrentou forte resistência inicial por trazer o tradutor automático, o compilador. Sobre o Custo: Escrever diretamente em assembly consumia muito tempo e gerava altos custos operacionais. O Fortran fez a redução do esforço humano possível ao apresentar fórmulas matemáticas de forma a notação natural. Sobre o Impacto: Apesar do sucesso da linguagem depender diretamente da eficiência do compilador, ao mostrar desempenho comparável ao código manual, o fortran remove sua principal barreira de sua forte resistência. Após isso, sua combinação de alta velocidade e produtividade garantiu rápida e massiva adoção pela comunidade científica e de engenharia.

5. Lisp surgiu em um contexto diferente de Fortran. Compare os domínios, a representação de dados e o estilo de computação favorecido pelas duas linguagens. 
R: Fortran era uma linguagem focada em programas de engenharia e científicas. Sua representação de dados era focada em números e vetores/matrizes estáticas, e seu estilo de computação era imperativo. Lisp era focada em linguagens para inteligência artificial, processamento de linguagem natural e manipulação de símbolos. Sua representação é baseada em átomos e listas ligadas, onde o próprio código é representado como dados. Seu estilo de  computação gira entorno do estilo funcional.

8. Compare Basic e PL/I como respostas ao desejo de ampliar o acesso ou o alcance da programação. Qual compromisso de projeto aparece em cada caso?
R: O Compromisso do Basic foi focar na simplicidade de aprendizado para pessoas não especialistas, sacrificando algumas partes de suas estrutura. Já o compromisso do PL/I foi abranger os domínios de aplicação em uma única grande linguagem, fazendo o completo oposto do Basic. Ela era estruturada de forma para unifica todas as necessidades da época em apenas uma ferramenta.

10. Defina ortogonalidade no projeto de linguagens e use ALGOL 68 para discutir a diferença entre regularidade e simplicidade. Uma linguagem muito ortogonal é automaticamente fácil de usar?
R: Ortogonalidade é a propriedade que dita que os recursos de uma linguagem operem de maneira independente, além de permitir combinações lógicas primitivas. ALGOL 68 leva a ortogonalidade ao extremo, ilustrando bem a diferença de simplicidade e regularidade. A primeira é basicamente a facilidade para entender, ler e aprender. Já a regularidade, é a reunião de regras uniformes sem exceções, pois retornam valores e faz com que possam ser usadas em qualquer lugar onde um valor daquele tipo seja esperado. Uma linguagem ortogonal não é fácil de utilizar, já o processo de regularidade a torna difícil durante legibilidade, além disso, os compiladores se tornam muito difícies de implementar de forma eficiente.

14. Compare o papel dos objetos em Smalltalk, C++ e Java. Inclua na
resposta o compromisso de C++ com C e a estratégia de portabilidade
de Java.
R: 
Papel dos Objetos
	- Smalltalk (Orientação a Objetos Pura): Absolutamente tudo é um objeto, desde um número inteiro até o próprio sistema operacional. Não existem tipos primitivos. Toda a computação ocorre estritamente por meio de troca de mensagens entre objetos.
	- C++ (Abordagem Híbrida): Os objetos são extensões opcionais. A linguagem adota um modelo híbrido para manter o compromisso total de compatibilidade com o C. Para aceitar códigos legados e priorizar a eficiência, o C++ manteve tipos primitivos e ponteiros idênticos ao C, permitindo misturar programação procedural e orientada a objetos.
	- Java (Abordagem Baseada em Classes): Quase tudo é objeto, e toda função deve residir dentro de uma classe. Para eliminar a insegurança do C++, todos os objetos são alocados no heap e manipulados por referências controladas. Contudo, para preservar o desempenho, o Java manteve tipos primitivos (como int e char) fora da hierarquia de objetos.

	Diferente do C++, que compila direto para código de máquina específico, o Java usa uma estratégia baseada na Java Virtual Machine (JVM). O código-fonte é compilado em uma representação intermediária universal chamada bytecode. Como esse formato é independente de hardware, qualquer plataforma que possua uma JVM instalada pode executá-lo, garantindo a portabilidade do sistema.

15. A primeira aplicação de Java não foi a Web, mas a Web impulsionou
sua adoção. Explique como mudanças de contexto podem reposicionar
uma linguagem.
R: A primeira aplicação do Java foi no desenvolvimento de software para dispositivos eletrônicos de consumo (sob o projeto Oak), mas o surgimento simultâneo da World Wide Web gerou uma nova necessidade global.
	Mudanças de contexto mudam quais critérios de avaliação de linguagens são prioritários. O Java foi reposicionado porque os mesmos recursos criados para eletrodomésticos resolveram perfeitamente os maiores problemas da Web (portabilidade, confiabilidade e segurança).

16. Compare Perl, JavaScript, PHP, Python, Ruby e Lua usando três
eixos: domínio inicial, estruturas de dados e estratégia de
implementação. Evite concluir que todas são iguais por serem
chamadas de scripting.
R: Perl nasceu para o processamento de textos e relatórios em sistemas UNIX. Ele separa explicitamente suas estruturas de dados em escalares, arrays e hashes através de símbolos específicos e usa implementação híbrida, compilado o código rapidamente para uma representação intermediária antes de interpretá-lo
	Javascript foi criado para o domínio específico da Web client-side em navegadores. Suas estruturas e dados centram-se em objetos baseados em protótipos que operam como dicionário de propriedades. Suas estratégias de implementação evoluiu de uma interpretação pura no navegador para a compilação JIT de alta performance.
	PHP surgiu exclusivamente para a geração de páginas web dinâmicas no servidor. Ele unificou todas as suas coleções em uma única estrutura de dados nativas: o array associativo, que funciona simultaneamente como lista, hash e matriz. É tradicionalmente executado por interpretação pura como um módulo embutido no servidor web.
	O Python foi projetado para programação de propósito geral de alto nível, focando na legibilidade. Ele traz um conjunto expressivo de estruturas de dados nativas, divididas rigidamente entre listas mutáveis, tuplas imutáveis e dicionários. Sua implementação é híbrida, convertendo o código-fonte em bytecode executado por uma máquina virtual.
	O Ruby teve como domínio inicial a orientação a objetos pura de propósito geral. Diferente das outras, sua estrutura de dados dita que absolutamente tudo é um objeto, de inteiros a arrays. Sua implementação original era baseada em interpretação pura de árvore sintática, migrando posteriormente para abordagens com bytecode.
	A Lua foi desenvolvida como uma linguagem extensível de configuração e suporte para sistemas em C/C++. Ela possui uma única e universal estrutura de dados chamada Tabela, usada para representar arrays, registros e objetos. Sua implementação é híbrida e focada em uma máquina virtual extremamente leve e compacta.

17. C# foi apresentada como evolução no ambiente .NET. Compare duas
decisões de C# com suas correspondentes em Java ou C++ e explique o
problema que pretendem resolver.
R: Duas decisões importantes foram o uso de Structs e os Rectangular Arrays
Structs
	- No Java, todos os tipos definidos pelo usuário (classes) são obrigatoriamente alocados na memória dinâmica (heap) e manipulados via referências. Sebesta aponta que isso gera um problema de desempenho e overhead de memória, pois criar pequenos agregados de dados (como um ponto tridimensional X, Y, Z) no Java exige alocação no heap e posterior atuação do Garbage Collector. O C# inclui o construtor struct, permitindo que o programador crie tipos de dados personalizados que são alocados diretamente na pilha (stack) e passados por valor . Além disso permiter estruturas leves e eficientes na pilha

Rectangular Arrays
	- Sebesta discute detalhadamente a estrutura de arrays na seção de tipos de dados. No Java, não existem matrizes retangulares reais; existem apenas arrays de arrays (jagged arrays ou matrizes dentadas). O problema das matrizes dentadas do Java é que cada linha é um objeto independente alocado separadamente no heap. Isso prejudica a localidade de referência da memória cache e torna o acesso a tabelas matemáticas significativamente mais lento. O C# resolve isso alocando a matriz como um bloco de memória único e contíguo, suportando nativamente arrays multidimensionais verdadeiros (retangulares), declarados como int[ , ] matriz = new int[3, 4];

18. Diferencie XSLT e JSP quanto a entrada, processamento e saída. Por
que ambas podem ser chamadas de linguagens híbridas de marcação e
programação?
R: 
XSLT
	Entrada: Recebe obrigatoriamente um documento XML contendo dados estruturados sem formatação visual.
	Processamento: O processador XSLT utiliza um arquivo de folha de estilo baseado em templates estruturados em padrões estruturais (como caminhos XPath). Ele varre a árvore do XML buscando correspondências e aplica as regras de transformação declarativas associadas a cada nó.
	Saída: Gera um novo documento de texto, que pode ser formatado em HTML para exibição em navegadores, outro arquivo XML ou arquivos de texto puro.

JSP
	Entrada: Recebe requisições HTTP enviadas pelo cliente a um servidor web combinadas com tags de marcação embutidas no próprio arquivo .jsp
	Processamento: O processador JSP transforma a página híbrida em um Servlet Java. Esse Servlet é compilado em bytecode e executado no lado do servidor, manipulando regras de negócio complexas, acesso a bancos de dados e lógica imperativa.
	Saída: Retorna um fluxo de código HTML puro dinâmico injetado na resposta HTTP e enviado diretamente para renderização no navegador do usuário.

	Ambas são classificadas como Linguagens Híbridas pois estendem a  sintaxe estrutural de uma linguagem de marcação tradicional para dar suporte na construção da programação lógica.

