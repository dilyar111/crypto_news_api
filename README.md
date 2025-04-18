# 📰 Crypto News Aggregator

A full-stack web application that aggregates the latest cryptocurrency news and real-time price data from multiple sources. Built with **Next.js**, **TypeScript**, and **Rust**, this project offers a responsive and modern interface for crypto enthusiasts to stay updated.

---

## 🚀 Features

- **Real-Time News Feed**: Fetches and displays the latest crypto news from various reputable sources.
- **Live Price Ticker**: Shows up-to-date cryptocurrency prices using external APIs.
- **Responsive Design**: Optimized for both desktop and mobile viewing.
- **Modular Architecture**: Clean separation between frontend and backend components for scalability.

---

## 🚀 How to Run

> ❗ First, install Rust and Cargo:
> https://www.rust-lang.org/tools/install

### 1. Clone the repository

```bash
git clone https://github.com/dilyar111/crypto_news_api.git
cd crypto_news_api
```

### 2. Run the server

```bash
cargo run
```

### 3. Server will be available at:

```cpp
[cargo run](http://127.0.0.1:8080)
```

## 🌐 API Endpoints

### ✅ Health Check

```vbnet
GET /health
Response: "Server is running"
```

### 📈 Get All Cryptos

```bash
GET /news
```

### 🔍 Search Crypto by name or symbol

```sql
GET /news?search=btc
```


### 📦 Example Response:
```json
[
  {
    "title": "Bitcoin (btc-bitcoin)",
    "source": "CoinPaprika",
    "date": "coin",
    "summary": "Price: $68942.21, Market Cap: $1349.22B",
    "url": "https://coinpaprika.com/coin/btc-bitcoin/"
  }
]

```

## 🖼️ Screenshots

![2025-04-18 11 15 02](https://github.com/user-attachments/assets/cb0986df-f0f7-4f23-9716-501391b7b55c)

![2025-04-18 11 15 08](https://github.com/user-attachments/assets/07b55561-32f5-4d94-ad73-8dbf3299d87a)



## 🛠️ Tech Stack

- **Frontend**: [Next.js](https://nextjs.org/) with TypeScript  
- **Backend**: Rust (for high-performance data processing)  
- **Styling**: CSS Modules  
- **APIs**:
  - [NewsDataHub](https://newsdatahub.com/) for news aggregation
  - [CoinGecko](https://www.coingecko.com/) for cryptocurrency price data

---

## 📄 License

Licensed under the [MIT License](https://github.com/dilyar111/crypto_news_api/blob/main/LICENSE).

