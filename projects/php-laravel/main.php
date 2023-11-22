<?php

// Funzione per ottenere il percorso corrente della directory
function getCurrentDirectory() {
    return getcwd();
}

// Recupera il percorso corrente
$currentDirectory = getCurrentDirectory();

// Verifica se l'URL richiesto corrisponde a uno degli endpoint
if ($_SERVER['REQUEST_METHOD'] === 'GET') {
    $url = parse_url($_SERVER['REQUEST_URI'], PHP_URL_PATH);

    switch ($url) {
        case '/small':
            // Restituisci il contenuto del file JSON per l'endpoint 1
            header('Content-Type: application/json');
            readfile('../../file-test/small.json');
            break;

        case '/medium':
            // Restituisci il contenuto del file JSON per l'endpoint 2
            header('Content-Type: application/json');
            readfile('../../file-test/medium.json');
            break;

        case '/large':
            // Restituisci il contenuto del file JSON per l'endpoint 3
            header('Content-Type: application/json');
            readfile('../../file-test/large.json');
            break;

        default:
            // Se l'URL non corrisponde a nessuno degli endpoint, restituisci il percorso corrente
            header('Content-Type: application/json');
            echo json_encode(['current_directory' => $currentDirectory]);
            break;
    }
} else {
    // Metodo non supportato
    header('HTTP/1.1 405 Method Not Allowed');
    header('Allow: GET');
    echo json_encode(['error' => 'Metodo non supportato']);
}

?>
