# src/locales/pt-PT.ftl

hero = Herói
    .learn = 📚 Aprender Dioxus
    .awesome = 🚀 Fantástico Dioxus
    .community = 📡 Bibliotecas da Comunidade
    .kit = ⚙️ Kit de Desenvolvimento Dioxus
    .code = 💫 Extensão para VSCode
    .discord = 👋 Discord da Comunidade

navbar = Navegação
    .home = Início
    .blog = Blog
    .dashboard = Painel de Controlo
    .theme-select = Selecionar Tema
    .language-select = Selecionar Idioma

blog = Este é o blog #{$id}
    .description = {$id ->
        [one] No primeiro blog, utilizamos os atributos e selectors do fluent.
       *[other] No blog #{$id}, mostramos como funciona o router Dioxus e como os parâmetros URL podem ser passados como props aos nossos componentes de rota.
    }

bu = button
    .prev = Anterior
    .next = Seguinte
    .close = Fechar

register = Criar Conta
    .suc = Conta com o email { $username } foi criada com sucesso.

login = Entrar
    .suc = Bem vindo { $username }.

logout = Sair
    .suc = A sessão foi terminda com sucesso.

frm-password = Palavra-passe
    .err = Deve ter mais de 8 caracteres, incluindo número, letra minúscula e letra maiúscula
    .invalid = Palavra-passe invalida

frm-email = E-mail
    .err = Deve introduzir um endereço de e-mail válido.
    .in-use = O e-mail fornecido está a ser usado.
    .free = O e-mail fornecido é válido.

unexpected = Oops, encontrámos um erro. Por favor, relate isto ao programador desta aplicação.
