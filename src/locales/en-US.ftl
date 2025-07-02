# src/locales/en-US.ftl
hero = Hero
    .learn = 📚 Learn Dioxus
    .awesome = 🚀 Awesome Dioxus
    .community = 📡 Community Libraries
    .kit = ⚙️ Dioxus Development Kit
    .code = 💫 VSCode Extension
    .discord = 👋 Community Discord

navbar = navbar
    .home = Home
    .blog = Blog
    .theme-select = Select Theme
    .language-select = Select Language

blog = This is blog #{$id}
    .description = {$id ->
        [one] In the first blog, we use fluent attributes and selectors.
       *[other] In blog #{$id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components.
    }

bu = button
    .prev = Previous
    .next = Next
    .close = Close

user = User
    .not-found = User not found

register = Create Account
    .suc = Account with email { $username } was created with success.

login = Login
    .suc = Welcome { $username }.
    .required = Login required

logout = Logout
    .suc = Your session was terminated.

frm-password = Password
    .err = Must be more than 8 characters, including number, lowercase letter, uppercase letter
    .invalid = Invalid password
    .change = Change Password
    .old = Old Password
    .new = New Password
    .suc-change = Your password was changed.

frm-email = Email
    .err = Must enter a valid email address.
    .in-use = The email provided it's in use.
    .free = The email provided it's valid.

unexpected = Oops, we encountered an error. Please report this to the developer of this application.

unauthorized = Unauthorized
forbidden = Forbidden: You do not have permission to access this resource.
