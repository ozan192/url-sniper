module.exports = {
  apps: [
    {
      name: "rust-app",                // Uygulamanızın adı
      script: "./start.js",             // Node.js wrapper dosyanız
      instances: 1,                     // Çalışacak instance sayısı
      autorestart: true,                // Uygulama kapanırsa yeniden başlatılmasını sağlar
      watch: true,                      // Dosya değişikliklerinde yeniden başlatma
      max_memory_restart: "100M",       // Maksimum bellek sınırı
      env: {
        NODE_ENV: "development"         // Geliştirme ortamı
      },
      env_production: {
        NODE_ENV: "production"          // Üretim ortamı
      }
    }
  ]
};
