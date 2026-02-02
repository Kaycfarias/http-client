# HTTP Client - Projeto Completo

Um cliente HTTP completo construído em Rust usando o framework Iced para a interface gráfica.

## ✨ Features Implementadas

### Features Essenciais (MVP) ✅

- **✅ Métodos HTTP Completos**
  - GET, POST, PUT, PATCH, DELETE
  - Seleção via dropdown

- **✅ Campo de URL + Validação**
  - Input de URL com validação em tempo real
  - Auto-adição de `https://` se não especificado
  - Mensagens de erro claras

- **✅ Headers Customizáveis**
  - Lista key/value dinâmica
  - Toggle on/off por header
  - Adicionar/remover headers facilmente
  - Header `Content-Type: application/json` por padrão

- **✅ Body da Requisição**
  - Suporte para Raw text
  - Suporte para JSON
  - Opção de body vazio
  - Bloqueio automático do body para requisições GET

- **✅ Enviar Request**
  - Botão "Send" com feedback visual
  - Estado de loading
  - Bloqueio de múltiplos envios simultâneos

- **✅ Resposta da API**
  - Status code com cor (verde=2xx, vermelho=4xx/5xx)
  - Tempo de resposta em ms
  - Body da resposta com scroll
  - Headers da resposta
  - Tabs para alternar entre Body e Headers

### Features Avançadas ✅

- **✅ Viewer de JSON**
  - Pretty print automático
  - Formatação com indentação
  - Detecção automática de JSON

- **✅ Histórico de Requests**
  - **Persistência**: Salvo automaticamente em disco (JSON)
  - **Auto-load**: Carrega histórico ao iniciar aplicação
  - Armazena últimas 50 requisições
  - Mostra método + URL + tempo + timestamp
  - Clique para reutilizar request anterior
  - Botão para limpar histórico
  - **Location**: `~/.config/http-client/history.json` (Linux/macOS) ou `%APPDATA%\http-client\history.json` (Windows)

- **✅ Editor de Query Params**
  - Lista key/value
  - Auto-encode de parâmetros
  - Toggle on/off por parâmetro
  - Adicionar/remover dinamicamente

- **✅ Timeout Configurável**
  - Campo editável em milissegundos
  - Default: 30000ms (30 segundos)
  - Tratamento de timeout com mensagem clara

## 🏗️ Arquitetura do Projeto

### Estrutura de Pastas

```
src/
├── main.rs                 # Aplicação principal e UI
└── components/
    ├── mod.rs             # Módulo raiz
    ├── enums.rs           # Tipos e estruturas de dados
    ├── http_client.rs     # Cliente HTTP com validações
    ├── history.rs         # Gerenciamento de histórico
    ├── pick_list.rs       # Component para selecionar método HTTP
    └── utils.rs           # Utilitários (validação, formatação)
```

### Módulos Principais

#### `enums.rs` - Estruturas de Dados

**Nota**: Todas as structs principais implementam `Serialize` e `Deserialize` do serde para persistência.

- `HTTPMethod`: Enum para métodos HTTP (GET, POST, PUT, PATCH, DELETE)
- `KeyValue`: Estrutura para headers e query params (key, value, enabled)
- `BodyType`: Enum para tipos de body (None, Raw, Json)
- `HttpRequest`: Estrutura completa da requisição
- `HttpResponse`: Estrutura completa da resposta
- `HistoryItem`: Item do histórico (request + response + timestamp)
- `Message`: Todas as mensagens da aplicação (padrão Elm Architecture)
- `RequestTab` e `ResponseTab`: Enums para navegação por tabs

#### `http_client.rs` - Cliente HTTP

- **`HttpClient`**: Cliente HTTP principal
  - Método `send_request()`: Envia requisição assíncrona
  - `validate_and_normalize_url()`: Validação e normalização de URLs
  - `build_url_with_params()`: Construção de URL com query params
  - `build_headers()`: Construção de headers da requisição
  - `format_error()`: Formatação de erros com mensagens claras

#### `history.rs` - Gerenciamento de Histórico

- **`RequestHistory`**: Gerencia histórico de requisições com persistência
  - **Persistência**: Auto-save/load via JSON usando serde
  - **Localização**: Usa crate `dirs` para obter diretório de config do SO
  - Armazena últimas 50 requisições
  - `add_item()`: Adiciona nova requisição ao histórico e salva no disco
  - `get_items()`: Retorna lista de requisições
  - `clear()`: Limpa histórico e arquivo
  - `save_to_file()`: Serializa e salva histórico em JSON
  - `load_from_file()`: Carrega e desserializa histórico do disco
  - `format_timestamp()`: Formata timestamp para exibição

#### `utils.rs` - Utilitários

- **`url_validator`**: Validação e normalização de URLs
- **`json_formatter`**: Formatação e validação de JSON
  - `format()`: Pretty print de JSON
  - `is_valid_json()`: Verifica se string é JSON válido
  - `minify()`: Minifica JSON
- **`text_formatter`**: Formatação de texto
  - `format_duration()`: Formata duração (ms, s, min)
  - `format_bytes()`: Formata tamanho de bytes
- **`export`**: Exportação de dados
  - `to_curl()`: Converte request para comando curl
  - `headers_to_string()`: Formata headers

### Padrões de Código

#### Separação de Responsabilidades

- **UI** (`main.rs`): Apenas lógica de apresentação
- **Lógica de Negócio** (`http_client.rs`): HTTP e validações
- **Dados** (`enums.rs`): Estruturas de dados centralizadas
- **Utilitários** (`utils.rs`): Funções helper reutilizáveis

#### Tratamento de Erros

- `Result<T, String>` para operações que podem falhar
- Mensagens de erro amigáveis e específicas
- Validação antes de enviar requisição

#### Validações Implementadas

- ✅ URL não vazia
- ✅ URL válida (com auto-correção de protocolo)
- ✅ Timeout configurável
- ✅ Headers habilitados/desabilitados
- ✅ Query params habilitados/desabilitados
- ✅ Body apenas para métodos que suportam

## 🚀 Como Executar

```bash
# Compilar
cargo build --release

# Executar em modo dev
cargo run

# Executar versão otimizada
./target/release/http-client
```

## 🧪 Como Testar

### Teste Básico (GET)

1. Selecione método `GET`
2. Digite URL: `jsonplaceholder.typicode.com/posts/1`
3. Clique em `Send`
4. Veja a resposta formatada

### Teste com Query Params

1. Método `GET`
2. URL: `httpbin.org/get`
3. Na tab "Query Params", clique "+ Add Query Param"
4. Adicione: `test` = `value123`
5. Envie e veja os params na resposta

### Teste com Headers

1. Método `GET`
2. URL: `httpbin.org/headers`
3. Na tab "Headers", adicione custom headers
4. Envie e veja seus headers refletidos

### Teste com POST + JSON

1. Método `POST`
2. URL: `jsonplaceholder.typicode.com/posts`
3. Na tab "Body", selecione "JSON"
4. Cole: `{"title": "Test", "body": "Test body", "userId": 1}`
5. Envie e veja a resposta

## 📦 Dependências

```toml
[dependencies]
iced = "0.14.0"                           # Framework UI
reqwest = { version = "0.13.1", features = ["json"] }  # Cliente HTTP
tokio = { version = "1.49.0", features = ["full"] }    # Runtime async
serde = { version = "1.0", features = ["derive"] }     # Serialização
serde_json = "1.0"                        # JSON parser
url = "2.5"                               # Parsing e validação de URLs
chrono = "0.4"                            # Manipulação de datas/timestamps
```

## 🎯 Próximas Features (Opcional)

### Features "WOW" que podem ser adicionadas:

- **Coleções**: Agrupar e salvar requests relacionadas
- **Autenticação**: Bearer Token, Basic Auth, API Key
- **Exportar para cURL**: Copiar request como comando curl
- **Temas**: Claro/Escuro com persistência
- **Ambientes**: Dev/Staging/Prod com variáveis
- **Persistência**: Salvar histórico em arquivo

## 🏆 Pontos Fortes do Projeto

### Para Portfólio

✅ **Arquitetura limpa** - Fácil de entender e manter  
✅ **Separação de responsabilidades** - Cada módulo tem um propósito claro  
✅ **Tratamento de erros robusto** - Validações e mensagens claras  
✅ **UI funcional e intuitiva** - Tabs, loading states, feedback visual  
✅ **Funcionalidades completas** - Não é um projeto "pela metade"  
✅ **Código bem documentado** - Comentários e estrutura clara  
✅ **Padrões modernos** - Async/await, type safety, pattern matching

### Para Entrevistas

- Demonstra conhecimento de **Rust** (ownership, traits, async)
- Demonstra conhecimento de **HTTP** (métodos, headers, status codes)
- Demonstra **arquitetura de software** (modularização, separação de concerns)
- Demonstra **UX thinking** (validações, feedback, estados de loading)
- Demonstra capacidade de **completar** um projeto do início ao fim

## 📝 Notas Técnicas

### Arquitetura Elm (TEA)

O projeto usa o padrão The Elm Architecture implementado pelo Iced:

- **Model** (`App` struct): Estado da aplicação
- **Update** (`update()` method): Lógica de atualização
- **View** (`view()` method): Renderização da UI

### Async/Await

- Requisições HTTP são assíncronas usando `tokio`
- `Task::perform` transforma async em messages do Iced
- Não bloqueia a UI durante requisições

### Type Safety

- Enums para prevenir estados inválidos
- Pattern matching exaustivo
- Compilador garante tratamento de todos os casos

## 🤝 Contribuindo

Este projeto está pronto para uso e pode servir como base para:

- Aprendizado de Rust
- Aprendizado de Iced framework
- Base para projetos similares
- Referência de arquitetura

## 📄 Licença

MIT License - Use livremente para aprendizado e portfólio!

---

**Autor**: Desenvolvido como projeto de portfólio  
**Stack**: Rust + Iced + Reqwest + Tokio  
**Tipo**: Desktop Application (Cross-platform)
