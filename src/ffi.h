namespace rust_ffi {
    typedef void (*FrameworkViewOverrideMethod)(void* framework_view);
    struct FrameworkView {
        void Initialize();
        void Load();
        void Run();
        void SetWindow();
        void Uninitialize();
    private:
        void* framework_view;
        struct {
            FrameworkViewOverrideMethod initialize;
            FrameworkViewOverrideMethod load;
            FrameworkViewOverrideMethod run;
            FrameworkViewOverrideMethod set_window;
            FrameworkViewOverrideMethod uninitialize;
        } v_table;
    };
};
