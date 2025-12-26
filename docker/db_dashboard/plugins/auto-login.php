<?php

class AdminerAutoLogin
{
    protected $auth = array("driver" => "", "server" => "", "username" => "", "password" => "", "db" => "");

    function __construct()
    {
        $this->store_auth();
        if ($_SERVER["REQUEST_URI"] == "/") {
            $_POST["auth"] = $this->auth;
        }
    }

    function store_auth()
    {
        $this->auth["server"] = getenv("ADMINER_DEFAULT_SERVER");

        $this->auth["driver"] = getenv("ADMINER_AUTOLOGIN_DRIVER");

        $this->auth["username"] = getenv("ADMINER_AUTOLOGIN_USER");
        $this->auth["password"] = getenv("ADMINER_AUTOLOGIN_PASSWORD");

        $this->auth["db"] = getenv("ADMINER_AUTOLOGIN_DB");
    }

    function loginForm()
    {
        echo '<div class="error" style="margin-bottom: 15px; padding-top: 5px; padding-bottom: 5px;">Autologin is enabled. Use this only in development environemnt.</div>';
    }

    function loginFormField($name, $heading, $value)
    {
        if ($name == 'driver' && $this->auth["driver"] && strlen($this->auth["driver"]) > 0) {
            return '
                <tr>
                    <th>System</th>
                    <td>
                        <input name="auth[driver]" type="text" readonly value="' . $this->auth["driver"] . '" />
                    </td>
                </tr>
            ';
        }

        if ($name == 'server' && $this->auth["server"] && strlen($this->auth["server"]) > 0) {
            return '
                <tr>
                    <th>Server</th>
                    <td>
                        <input name="auth[server]" type="text" readonly value="' . $this->auth["server"] . '" />
                    </td>
                </tr>
            ';
        }

        if ($name == 'username' && $this->auth["username"] && strlen($this->auth["username"]) > 0) {
            return '
                <tr>
                    <th>Username</th>
                    <td>
                        <input name="auth[username]" type="text" readonly value="' . $this->auth["username"] . '" />
                    </td>
                </tr>
            ';
        }

        if ($name == 'password' && $this->auth["password"] && strlen($this->auth["password"]) > 0) {
            return '
                <tr>
                    <th>Password</th>
                    <td>
                        <input name="auth[password]" type="password" readonly value="' . $this->auth["password"] . '" />
                    </td>
                </tr>
            ';
        }

        if ($name == 'db' && $this->auth["db"] && strlen($this->auth["db"]) > 0) {
            return '
                <tr>
                    <th>Database</th>
                    <td>
                        <input name="auth[db]" type="text" readonly value="' . $this->auth["db"] . '" />
                    </td>
                </tr>
            ';
        }
    }


    function navigation() {
        echo '<div class="error" style="margin-bottom: 15px; padding-top: 5px; padding-bottom: 5px;">Autologin is enabled. Use this only in development environemnt.</div>';
    }
    // // Optional: Set default login credentials
    // function credentials() {
    //     return array($this->server, $this->username, $_POST['auth']['password'] ?? '');
    // }

    // // Optional: Set default database
    // function database() {
    //     return $this->database;
    // }

    // // Optional: Change login form field order/visibility
    // function loginFormField($name, $heading, $value) {
    //     // Hide certain fields if they're pre-filled
    //     if ($name === 'server' && $this->server) {
    //         return '<input type="hidden" name="auth[server]" value="' . h($this->server) . '">';
    //     }
    //     if ($name === 'username' && $this->username) {
    //         return '<input type="hidden" name="auth[username]" value="' . h($this->username) . '">';
    //     }
    //     return ''; // Use default rendering
    // }
}
