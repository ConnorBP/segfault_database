module.exports = {
    apps : [
        {
          name: "SegfaultDatabase",
          script: "./sfdb_run.sh",
          interpreter: "bash",
          watch: true,
          env: {
              "DATABASE_URL": "mysql://segfault_users:password@127.0.0.1:3306/segfault_users",
          }
        }
    ]
}